// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::BoundingBox;

/// Representation of an Ellipse.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct Ellipse {
    /// x-coordinate of center
    pub(crate) center_x: f64,
    /// y-coordinate of center
    pub(crate) center_y: f64,
    /// major axis squared
    pub(crate) major_axis: f64,
    /// minor axis squared
    pub(crate) minor_axis: f64,
    /// theta in degrees
    pub(crate) theta: f64,
}

impl Ellipse {
    /// Constructs a new ellipse.
    ///
    /// The canvas for defining ellipses is square and ranges from -1 to 1 on both axes `x` and `y`.
    ///
    /// # Parameters
    ///
    /// * `center_x`: x component of center on the canvas
    /// * `center_y`: y component of center on the canvas
    /// * `major_axis`: major axis length
    /// * `minor_axis`: minor axis length
    /// * `theta`: Rotation angle of the ellipse in degrees
    pub(crate) fn new(
        center_x: f64,
        center_y: f64,
        major_axis: f64,
        minor_axis: f64,
        theta: f64,
    ) -> Self {
        Ellipse {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            theta,
        }
    }

    #[inline(always)]
    pub(crate) fn on_canvas(&self, nx: u32, ny: u32) -> EllipseOnCanvas {
        let Self {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            theta,
        } = self;

        let theta = theta.to_radians();
        let theta_sin = theta.sin();
        let theta_cos = theta.cos();
        let nx_f = f64::from(nx);
        let ny_f = f64::from(ny);
        let nx_half = nx_f / 2.0;
        let ny_half = ny_f / 2.0;
        let n_min = std::cmp::min_by(nx_half, ny_half, |nx, ny| nx.partial_cmp(ny).unwrap());

        let center_x = center_x * n_min + nx_half;
        let center_y = center_y * n_min + ny_half;
        let major_axis = major_axis * n_min;
        let minor_axis = minor_axis * n_min;
        let major_axis_squared = major_axis.powi(2);
        let minor_axis_squared = minor_axis.powi(2);

        let theta_pi2_sin = (theta + std::f64::consts::FRAC_PI_2).sin();
        let theta_pi2_cos = (theta + std::f64::consts::FRAC_PI_2).cos();
        let ux = major_axis * theta_cos;
        let uy = major_axis * theta_sin;
        let vx = minor_axis * theta_pi2_cos;
        let vy = minor_axis * theta_pi2_sin;
        let halfwidth = (ux.powi(2) + vx.powi(2)).sqrt();
        let halfheight = (uy.powi(2) + vy.powi(2)).sqrt();

        let bbox: Vec<u32> = [
            ((center_x - halfwidth).floor(), nx_f),
            ((center_x + halfwidth).ceil(), nx_f),
            ((center_y - halfheight).floor(), ny_f),
            ((center_y + halfheight).ceil(), ny_f),
        ]
        .into_iter()
        .map(|(b, l)| {
            if b < 0.0 {
                0
            } else if b >= l {
                (l - 1.0) as u32
            } else {
                b as u32
            }
        })
        .collect();

        EllipseOnCanvas {
            center_x,
            center_y,
            major_axis_squared,
            minor_axis_squared,
            theta_sin,
            theta_cos,
            bbox: (bbox[0], bbox[1], bbox[2], bbox[3]).into(),
        }
    }
}

/// Representation of an Ellipse.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct EllipseOnCanvas {
    /// x-coordinate of center
    center_x: f64,
    /// y-coordinate of center
    center_y: f64,
    /// major axis squared
    major_axis_squared: f64,
    /// minor axis squared
    minor_axis_squared: f64,
    /// sin(theta)
    theta_sin: f64,
    /// cos(theta)
    theta_cos: f64,
    /// bounding box
    bbox: BoundingBox,
}

impl EllipseOnCanvas {
    #[inline(always)]
    pub(crate) fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }

    /// Checks if a point is inside the ellipse
    #[inline(always)]
    pub(crate) fn inside(&self, x: f64, y: f64) -> bool {
        let x_diff = x - self.center_x;
        let y_diff = y - self.center_y;
        (self.theta_cos * x_diff + self.theta_sin * y_diff).powi(2) / self.major_axis_squared
            + (self.theta_sin * x_diff - self.theta_cos * y_diff).powi(2) / self.minor_axis_squared
            <= 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::Ellipse;
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
    fn test_ellipse_new(
        center_x_input: FloatNotNan,
        center_y_input: FloatNotNan,
        major_axis_input: FloatNotNan,
        minor_axis_input: FloatNotNan,
        theta_input: FloatNotNan,
    ) -> bool {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let major_axis_input = major_axis_input.0;
        let minor_axis_input = minor_axis_input.0;
        let theta_input = theta_input.0;
        let Ellipse {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            theta,
        } = Ellipse::new(
            center_x_input,
            center_y_input,
            major_axis_input,
            minor_axis_input,
            theta_input,
        );
        center_x.to_ne_bytes() == center_x_input.to_ne_bytes()
            && center_y.to_ne_bytes() == center_y_input.to_ne_bytes()
            && major_axis.to_ne_bytes() == major_axis_input.to_ne_bytes()
            && minor_axis.to_ne_bytes() == minor_axis_input.to_ne_bytes()
            && theta.to_ne_bytes() == theta_input.to_ne_bytes()
    }

    #[quickcheck]
    fn test_from_ellipse_to_ellpiseoncanvas(
        center_x_input: FloatLim,
        center_y_input: FloatLim,
        major_axis_input: FloatLim,
        minor_axis_input: FloatLim,
        theta_input: FloatLim,
        nx: u32,
        ny: u32,
    ) {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let major_axis_input = major_axis_input.0;
        let minor_axis_input = minor_axis_input.0;
        let theta_input = theta_input.0;

        let ellipse = Ellipse::new(
            center_x_input,
            center_y_input,
            major_axis_input,
            minor_axis_input,
            theta_input,
        );

        let ellipse_on_canvas = ellipse.on_canvas(nx, ny);

        let theta = ellipse.theta.to_radians();
        let theta_sin = theta.sin();
        let theta_cos = theta.cos();
        let nx_f = f64::from(nx);
        let ny_f = f64::from(ny);
        let nx_half = nx_f / 2.0;
        let ny_half = ny_f / 2.0;
        let n_min = std::cmp::min_by(nx_half, ny_half, |nx, ny| nx.partial_cmp(ny).unwrap());

        let center_x = ellipse.center_x * n_min + nx_half;
        let center_y = ellipse.center_y * n_min + ny_half;
        let major_axis = ellipse.major_axis * n_min;
        let minor_axis = ellipse.minor_axis * n_min;
        let major_axis_squared = major_axis.powi(2);
        let minor_axis_squared = minor_axis.powi(2);

        let theta_pi2_sin = (theta + std::f64::consts::FRAC_PI_2).sin();
        let theta_pi2_cos = (theta + std::f64::consts::FRAC_PI_2).cos();
        let ux = major_axis * theta_cos;
        let uy = major_axis * theta_sin;
        let vx = minor_axis * theta_pi2_cos;
        let vy = minor_axis * theta_pi2_sin;
        let halfwidth = (ux.powi(2) + vx.powi(2)).sqrt();
        let halfheight = (uy.powi(2) + vy.powi(2)).sqrt();

        let bbox: Vec<u32> = [
            ((center_x - halfwidth).floor(), nx_f),
            ((center_x + halfwidth).ceil(), nx_f),
            ((center_y - halfheight).floor(), ny_f),
            ((center_y + halfheight).ceil(), ny_f),
        ]
        .into_iter()
        .map(|(b, l)| {
            if b < 0.0 {
                0
            } else if b >= l {
                (l - 1.0) as u32
            } else {
                b as u32
            }
        })
        .collect();

        assert_abs_diff_eq!(ellipse_on_canvas.center_x, center_x);
        assert_abs_diff_eq!(ellipse_on_canvas.center_y, center_y);
        assert_abs_diff_eq!(ellipse_on_canvas.major_axis_squared, major_axis_squared);
        assert_abs_diff_eq!(ellipse_on_canvas.minor_axis_squared, minor_axis_squared);
        assert_abs_diff_eq!(ellipse_on_canvas.theta_sin, theta_sin);
        assert_abs_diff_eq!(ellipse_on_canvas.theta_cos, theta_cos);
        assert_eq!(
            ellipse_on_canvas.bbox,
            (bbox[0], bbox[1], bbox[2], bbox[3]).into()
        );
        assert_eq!(
            ellipse_on_canvas.bounding_box(),
            (bbox[0], bbox[1], bbox[2], bbox[3]).into()
        );
    }

    #[quickcheck]
    // Add a reason why this lint is allowed once the feature `lint_reasons` is stabilized.
    #[allow(clippy::too_many_arguments)]
    fn test_inside(
        center_x: FloatLim,
        center_y: FloatLim,
        minor_axis: FloatLim,
        theta: FloatLim,
        nx: u32,
        ny: u32,
        x: FloatNotNan,
        y: FloatNotNan,
    ) -> bool {
        let x = x.0;
        let y = y.0;
        let major_axis = 2.0 * minor_axis.0;

        let ellipse = Ellipse::new(center_x.0, center_y.0, major_axis, minor_axis.0, theta.0)
            .on_canvas(nx, ny);

        let x_diff = x - ellipse.center_x;
        let y_diff = y - ellipse.center_y;
        let inside = (ellipse.theta_cos * x_diff + ellipse.theta_sin * y_diff).powi(2)
            / ellipse.major_axis_squared
            + (ellipse.theta_sin * x_diff - ellipse.theta_cos * y_diff).powi(2)
                / ellipse.minor_axis_squared
            <= 1.0;

        ellipse.inside(x, y) == inside
    }
}
