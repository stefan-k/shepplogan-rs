// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::BoundingBox;

/// Representation of a Rectangle
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Rectangle {
    /// x-coordinate of center
    pub(crate) center_x: f64,
    /// y-coordinate of center
    pub(crate) center_y: f64,
    /// width
    pub(crate) width: f64,
    /// height
    pub(crate) height: f64,
    /// rotation
    pub(crate) theta: f64,
}

impl Rectangle {
    /// Constructs a new rectangle.
    ///
    /// The canvas for defining rectangles is square and ranges from -1 to 1 on both axes `x` and `y`.
    ///
    /// # Parameters
    ///
    /// * `center_x`: x component of center on the canvas
    /// * `center_y`: y component of center on the canvas
    /// * `width`: width of the rectangle
    /// * `height`: height of the rectangle
    /// * `theta`: Rotation angle of the rectangle in degrees
    pub(crate) fn new(center_x: f64, center_y: f64, width: f64, height: f64, theta: f64) -> Self {
        Rectangle {
            center_x,
            center_y,
            width,
            height,
            theta,
        }
    }

    #[inline(always)]
    pub(crate) fn on_canvas(&self, nx: u32, ny: u32) -> RectangleOnCanvas {
        let Self {
            center_x,
            center_y,
            width,
            height,
            theta,
        } = self;

        let theta = theta.to_radians();
        let theta_sin = theta.sin();
        let theta_cos = theta.cos();
        let nx_f = f64::from(nx);
        let ny_f = f64::from(ny);
        let nx_half = nx_f / 2.0;
        let ny_half = ny_f / 2.0;
        // Everything is going to be scaled by the smaller dimension
        let n_min = std::cmp::min_by(nx_half, ny_half, |nx, ny| nx.partial_cmp(ny).unwrap());

        let width_half = width / 2.0;
        let height_half = height / 2.0;

        // Compute the corner points (unrotated)
        let a_x = center_x - width_half;
        let a_y = center_y - height_half;
        let b_x = center_x - width_half;
        let b_y = center_y + height_half;
        let c_x = center_x + width_half;
        let c_y = center_y + height_half;
        let d_x = center_x + width_half;
        let d_y = center_y - height_half;

        // Rotate the points
        let rotate =
            |x: f64, y: f64| (x * theta_cos - y * theta_sin, x * theta_sin + y * theta_cos);

        let (a_xr, a_yr) = rotate(a_x, a_y);
        let (b_xr, b_yr) = rotate(b_x, b_y);
        let (c_xr, c_yr) = rotate(c_x, c_y);
        let (d_xr, d_yr) = rotate(d_x, d_y);

        // Now scale and shift them onto the new canvas
        let scale_shift = |x: f64, y: f64| (x * n_min + nx_half, y * n_min + ny_half);

        let (a_xr, a_yr) = scale_shift(a_xr, a_yr);
        let (b_xr, b_yr) = scale_shift(b_xr, b_yr);
        let (c_xr, c_yr) = scale_shift(c_xr, c_yr);
        let (d_xr, d_yr) = scale_shift(d_xr, d_yr);

        // compute the minimum and maximum coordinates for the bounding box.
        let min_max = |arr: &[f64]| {
            (
                arr.iter()
                    .cloned()
                    .map(f64::floor)
                    .map(|x| if x < 0.0 { 0.0 } else { x })
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap() as u32,
                arr.iter()
                    .cloned()
                    .map(f64::ceil)
                    .map(|x| if x > nx_f { nx_f } else { x })
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap() as u32,
            )
        };

        let (x_min, x_max) = min_max(&[a_xr, b_xr, c_xr, d_xr]);
        let (y_min, y_max) = min_max(&[a_yr, b_yr, c_yr, d_yr]);

        // Helper variables to make computing whether a point is inside or not easier later on
        let ab = (b_xr - a_xr, b_yr - a_yr);
        let bc = (c_xr - b_xr, c_yr - b_yr);
        let a = (a_xr, a_yr);
        let b = (b_xr, b_yr);
        let abab = ab.0.powi(2) + ab.1.powi(2);
        let bcbc = bc.0.powi(2) + bc.1.powi(2);

        RectangleOnCanvas {
            a,
            b,
            ab,
            bc,
            abab,
            bcbc,
            bbox: (x_min, x_max, y_min, y_max).into(),
        }
    }
}

/// Representation of a Rectangle on a canvas
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct RectangleOnCanvas {
    a: (f64, f64),
    b: (f64, f64),
    ab: (f64, f64),
    bc: (f64, f64),
    abab: f64,
    bcbc: f64,
    /// bounding box
    bbox: BoundingBox,
}

impl RectangleOnCanvas {
    #[inline(always)]
    pub(crate) fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }

    /// Checks if a point is inside the rectangle
    #[inline(always)]
    pub(crate) fn inside(&self, x: f64, y: f64) -> bool {
        let am = (x - self.a.0, y - self.a.1);
        let bm = (x - self.b.0, y - self.b.1);
        let abam = self.ab.0 * am.0 + self.ab.1 * am.1;
        let bcbm = self.bc.0 * bm.0 + self.bc.1 * bm.1;
        0.0 <= abam && abam <= self.abab && 0.0 <= bcbm && bcbm <= self.bcbc
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use approx::assert_abs_diff_eq;

    #[derive(Debug, Copy, Clone)]
    struct FloatLim(f64);

    impl quickcheck::Arbitrary for FloatLim {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g) % 100000.0;
                if !val.is_nan() && val.is_finite() {
                    return FloatLim(val);
                }
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct FloatNotNan(f64);

    impl quickcheck::Arbitrary for FloatNotNan {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g);
                if !val.is_nan() && val.is_finite() {
                    return FloatNotNan(val);
                }
            }
        }
    }

    #[quickcheck]
    fn test_rectangle_new(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        width: FloatNotNan,
        height: FloatNotNan,
        theta_input: FloatNotNan,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let width = width.0;
        let height = height.0;
        let theta_input = theta_input.0;
        let rectangle1 = Rectangle::new(center_x_input, center_y_input, width, height, theta_input);
        let rectangle2 = Rectangle::new(center_x_input, center_y_input, width, height, theta_input);
        let Rectangle {
            center_x,
            center_y,
            width,
            height,
            theta,
        } = rectangle1;
        rectangle1 == rectangle2
            && center_x.to_ne_bytes() == center_x_input.to_ne_bytes()
            && center_y.to_ne_bytes() == center_y_input.to_ne_bytes()
            && width.to_ne_bytes() == width.to_ne_bytes()
            && height.to_ne_bytes() == height.to_ne_bytes()
            && theta.to_ne_bytes() == theta_input.to_ne_bytes()
    }

    #[quickcheck]
    fn test_from_rectangle_to_rectangleoncanvas(
        center_x_input: FloatLim,
        center_y_input: FloatLim,
        width: FloatLim,
        height: FloatLim,
        theta_input: FloatLim,
        nx: u32,
        ny: u32,
    ) {
        let center_x = center_x_input.0;
        let center_y = center_y_input.0;
        let width = width.0;
        let height = height.0;
        let theta = theta_input.0;

        let rectangle = Rectangle::new(center_x, center_y, width, height, theta);

        let rectangle_on_canvas = rectangle.on_canvas(nx, ny);
        let rectangle_on_canvas2 = rectangle_on_canvas.clone();

        let theta = rectangle.theta.to_radians();
        let theta_sin = theta.sin();
        let theta_cos = theta.cos();
        let nx_f = f64::from(nx);
        let ny_f = f64::from(ny);
        let nx_half = nx_f / 2.0;
        let ny_half = ny_f / 2.0;
        let n_min = std::cmp::min_by(nx_half, ny_half, |nx, ny| nx.partial_cmp(ny).unwrap());

        let width_half = width / 2.0;
        let height_half = height / 2.0;

        let a_x = center_x - width_half;
        let a_y = center_y - height_half;
        let b_x = center_x - width_half;
        let b_y = center_y + height_half;
        let c_x = center_x + width_half;
        let c_y = center_y + height_half;
        let d_x = center_x + width_half;
        let d_y = center_y - height_half;

        let rotate =
            |x: f64, y: f64| (x * theta_cos - y * theta_sin, x * theta_sin + y * theta_cos);

        let (a_xr, a_yr) = rotate(a_x, a_y);
        let (b_xr, b_yr) = rotate(b_x, b_y);
        let (c_xr, c_yr) = rotate(c_x, c_y);
        let (d_xr, d_yr) = rotate(d_x, d_y);

        let scale_shift = |x: f64, y: f64| (x * n_min + nx_half, y * n_min + ny_half);

        let (a_xr, a_yr) = scale_shift(a_xr, a_yr);
        let (b_xr, b_yr) = scale_shift(b_xr, b_yr);
        let (c_xr, c_yr) = scale_shift(c_xr, c_yr);
        let (d_xr, d_yr) = scale_shift(d_xr, d_yr);

        let min_max = |arr: &[f64]| {
            (
                arr.iter()
                    .cloned()
                    .map(f64::floor)
                    .map(|x| if x < 0.0 { 0.0 } else { x })
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap() as u32,
                arr.iter()
                    .cloned()
                    .map(f64::ceil)
                    .map(|x| if x > nx_f { nx_f } else { x })
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap() as u32,
            )
        };

        let (x_min, x_max) = min_max(&[a_xr, b_xr, c_xr, d_xr]);
        let (y_min, y_max) = min_max(&[a_yr, b_yr, c_yr, d_yr]);

        let ab = (b_xr - a_xr, b_yr - a_yr);
        let bc = (c_xr - b_xr, c_yr - b_yr);
        let a = (a_xr, a_yr);
        let b = (b_xr, b_yr);
        let abab = ab.0.powi(2) + ab.1.powi(2);
        let bcbc = bc.0.powi(2) + bc.1.powi(2);

        assert_eq!(rectangle_on_canvas, rectangle_on_canvas2);
        assert_abs_diff_eq!(rectangle_on_canvas.a.0, a.0);
        assert_abs_diff_eq!(rectangle_on_canvas.a.1, a.1);
        assert_abs_diff_eq!(rectangle_on_canvas.b.0, b.0);
        assert_abs_diff_eq!(rectangle_on_canvas.b.1, b.1);
        assert_abs_diff_eq!(rectangle_on_canvas.ab.0, ab.0);
        assert_abs_diff_eq!(rectangle_on_canvas.ab.1, ab.1);
        assert_abs_diff_eq!(rectangle_on_canvas.bc.0, bc.0);
        assert_abs_diff_eq!(rectangle_on_canvas.bc.1, bc.1);
        assert_abs_diff_eq!(rectangle_on_canvas.abab, abab);
        assert_abs_diff_eq!(rectangle_on_canvas.bcbc, bcbc);
        assert_eq!(
            rectangle_on_canvas.bbox,
            (x_min, x_max, y_min, y_max).into()
        );
        assert_eq!(
            rectangle_on_canvas.bounding_box(),
            (x_min, x_max, y_min, y_max).into()
        );
    }

    #[quickcheck]
    // Add a reason why this lint is allowed once the feature `lint_reasons` is stabilized.
    #[allow(clippy::too_many_arguments)]
    fn test_inside(
        center_x: FloatLim,
        center_y: FloatLim,
        width: FloatLim,
        theta: FloatLim,
        nx: u32,
        ny: u32,
        x: FloatNotNan,
        y: FloatNotNan,
    ) -> bool {
        let x = x.0;
        let y = y.0;
        let width = width.0;
        let height = 2.0 * width;

        let rectangle =
            Rectangle::new(center_x.0, center_y.0, width, height, theta.0).on_canvas(nx, ny);

        let am = (x - rectangle.a.0, y - rectangle.a.1);
        let bm = (x - rectangle.b.0, y - rectangle.b.1);
        let abam = rectangle.ab.0 * am.0 + rectangle.ab.1 * am.1;
        let bcbm = rectangle.bc.0 * bm.0 + rectangle.bc.1 * bm.1;
        let inside = 0.0 <= abam && abam <= rectangle.abab && 0.0 <= bcbm && bcbm <= rectangle.bcbc;

        rectangle.inside(x, y) == inside
    }
}
