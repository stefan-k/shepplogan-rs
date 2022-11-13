// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Representation of an Ellipse.
pub(super) struct Ellipse {
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
    bounding_box: (f64, f64, f64, f64),
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
    pub(super) fn new(
        center_x: f64,
        center_y: f64,
        major_axis: f64,
        minor_axis: f64,
        theta: f64,
    ) -> Self {
        let theta = theta.to_radians();
        let theta_sin = theta.sin();
        let theta_cos = theta.cos();
        let theta_pi2_sin = (theta + std::f64::consts::FRAC_PI_2).sin();
        let theta_pi2_cos = (theta + std::f64::consts::FRAC_PI_2).cos();
        let ux = major_axis * theta_cos;
        let uy = major_axis * theta_sin;
        let vx = minor_axis * theta_pi2_cos;
        let vy = minor_axis * theta_pi2_sin;
        let halfwidth = (ux.powi(2) + vx.powi(2)).sqrt();
        let halfheight = (uy.powi(2) + vy.powi(2)).sqrt();

        let bounding_box = (
            (center_x - halfwidth),
            (center_y - halfheight),
            (center_x + halfwidth),
            (center_y + halfheight),
        );

        let major_axis_squared = major_axis.powi(2);
        let minor_axis_squared = minor_axis.powi(2);

        Ellipse {
            center_x,
            center_y,
            major_axis_squared,
            minor_axis_squared,
            theta_sin,
            theta_cos,
            bounding_box,
        }
    }

    /// Checks if a point is inside the ellipse
    #[inline(always)]
    pub(super) fn inside(&self, x: f64, y: f64) -> bool {
        (self.theta_cos * (x - self.center_x) + self.theta_sin * (y - self.center_y)).powi(2)
            / self.major_axis_squared
            + (self.theta_sin * (x - self.center_x) - self.theta_cos * (y - self.center_y)).powi(2)
                / self.minor_axis_squared
            <= 1.0
    }

    /// Return the bounding box of the ellipse scaled to the canvas given by `nx` and `ny`.
    #[inline(always)]
    pub(super) fn bounding_box(&self, nx: u32, ny: u32) -> (u32, u32, u32, u32) {
        let nx_f64 = f64::from(nx) / 2.0;
        let ny_f64 = f64::from(ny) / 2.0;
        let n_min = std::cmp::min_by(nx_f64, ny_f64, |nx, ny| nx.partial_cmp(ny).unwrap());
        let bx1 = ((self.bounding_box.0) * n_min + nx_f64).floor();
        let by1 = ((self.bounding_box.1) * n_min + ny_f64).floor();
        let bx2 = ((self.bounding_box.2) * n_min + nx_f64).ceil();
        let by2 = ((self.bounding_box.3) * n_min + ny_f64).ceil();
        let out: Vec<u32> = [(bx1, nx), (by1, ny), (bx2, nx), (by2, ny)]
            .into_iter()
            .map(|(x, n)| {
                if x < 0.0 {
                    0
                } else if x >= f64::from(n) {
                    n - 1
                } else {
                    x as u32
                }
            })
            .collect();
        (out[0], out[1], out[2], out[3])
    }
}
#[cfg(test)]
mod tests {
    use super::Ellipse;
    use approx::assert_abs_diff_eq;

    #[derive(Debug, Copy, Clone)]
    struct Float(f64);

    impl quickcheck::Arbitrary for Float {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g).abs() % 1000000.0;
                if !val.is_nan() {
                    return Float(val);
                }
            }
        }
    }

    #[quickcheck]
    fn test_new(
        center_x_input: Float,
        center_y_input: Float,
        major_axis_input: Float,
        minor_axis_input: Float,
        theta_input: Float,
    ) {
        let center_x_input = center_x_input.0;
        let center_y_input = center_y_input.0;
        let major_axis_input = major_axis_input.0;
        let minor_axis_input = minor_axis_input.0;
        let theta_input = theta_input.0;
        let Ellipse {
            center_x,
            center_y,
            major_axis_squared,
            minor_axis_squared,
            theta_sin,
            theta_cos,
            bounding_box,
        } = Ellipse::new(
            center_x_input,
            center_y_input,
            major_axis_input,
            minor_axis_input,
            theta_input,
        );
        assert_abs_diff_eq!(center_x, center_x_input);
        assert_abs_diff_eq!(center_y, center_y_input);
        assert_abs_diff_eq!(major_axis_squared, major_axis_input.powi(2));
        assert_abs_diff_eq!(minor_axis_squared, minor_axis_input.powi(2));
        assert_abs_diff_eq!(theta_sin, theta_input.to_radians().sin());
        assert_abs_diff_eq!(theta_cos, theta_input.to_radians().cos());

        let theta = theta_input.to_radians();
        let theta_sin = theta.sin();
        let theta_cos = theta.cos();
        let theta_pi2_sin = (theta + std::f64::consts::FRAC_PI_2).sin();
        let theta_pi2_cos = (theta + std::f64::consts::FRAC_PI_2).cos();
        let ux = major_axis_input * theta_cos;
        let uy = major_axis_input * theta_sin;
        let vx = minor_axis_input * theta_pi2_cos;
        let vy = minor_axis_input * theta_pi2_sin;
        let halfwidth = (ux.powi(2) + vx.powi(2)).sqrt();
        let halfheight = (uy.powi(2) + vy.powi(2)).sqrt();

        let bounding_box_true = (
            (center_x - halfwidth),
            (center_y - halfheight),
            (center_x + halfwidth),
            (center_y + halfheight),
        );

        assert_abs_diff_eq!(bounding_box.0, bounding_box_true.0);
        assert_abs_diff_eq!(bounding_box.1, bounding_box_true.1);
        assert_abs_diff_eq!(bounding_box.2, bounding_box_true.2);
        assert_abs_diff_eq!(bounding_box.3, bounding_box_true.3);
    }
}
