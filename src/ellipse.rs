// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Ellipse
//!
//! Create ellipses...?

/// Ellipse
pub struct Ellipse {
    /// x-coordinate of center
    center_x: f64,
    /// y-coordinate of center
    center_y: f64,
    /// major axis squared
    major_axis_squared: f64,
    /// minor axis
    minor_axis_squared: f64,
    /// sin(theta)
    theta_sin: f64,
    /// cos(theta)
    theta_cos: f64,
    /// intensity
    intensity: f64,
    /// bounding box
    bounding_box: (f64, f64, f64, f64),
}

impl Ellipse {
    /// Constructor
    pub fn new(
        center_x: f64,
        center_y: f64,
        major_axis: f64,
        minor_axis: f64,
        theta: f64,
        intensity: f64,
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
            intensity,
            bounding_box,
        }
    }

    /// Checks if a point is inside the ellipse
    #[inline(always)]
    pub fn inside(&self, x: f64, y: f64) -> bool {
        let y_shifted = y - self.center_y;
        let x_shifted = x - self.center_x;
        (self.theta_cos * x_shifted + self.theta_sin * y_shifted).powi(2) / self.major_axis_squared
            + (self.theta_sin * x_shifted - self.theta_cos * y_shifted).powi(2)
                / self.minor_axis_squared
            <= 1.0
    }

    /// Checks if a point is inside the ellipse
    #[inline(always)]
    pub fn inside_2(&self, x: f64, y: f64) -> f64 {
        let y_shifted = y - self.center_y;
        let x_shifted = x - self.center_x;
        if (self.theta_cos * x_shifted + self.theta_sin * y_shifted).powi(2)
            / self.major_axis_squared
            + (self.theta_sin * x_shifted - self.theta_cos * y_shifted).powi(2)
                / self.minor_axis_squared
            <= 1.0
        {
            self.intensity
        } else {
            0.0
        }
    }

    /// Return intensity of the ellipse
    #[inline(always)]
    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    /// Return the bounding box of the ellipse
    #[inline(always)]
    pub fn bounding_box(&self, nx: u32, ny: u32) -> (u32, u32, u32, u32) {
        let nx_f64 = f64::from(nx) / 2.0;
        let ny_f64 = f64::from(ny) / 2.0;
        let n_min = f64::from(std::cmp::min(nx, ny)) / 2.0;
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

    /// Return the bounding box of the ellipse
    #[inline(always)]
    pub fn bounding_box_usize(&self, nx: usize, ny: usize) -> (usize, usize, usize, usize) {
        let nx_f64 = f64::from(nx as u32) / 2.0;
        let ny_f64 = f64::from(ny as u32) / 2.0;
        let n_min = f64::from(std::cmp::min(nx, ny) as u32) / 2.0;
        let bx1 = ((self.bounding_box.0) * n_min + nx_f64).floor();
        let by1 = ((self.bounding_box.1) * n_min + ny_f64).floor();
        let bx2 = ((self.bounding_box.2) * n_min + nx_f64).ceil();
        let by2 = ((self.bounding_box.3) * n_min + ny_f64).ceil();
        let out: Vec<usize> = [(bx1, nx), (by1, ny), (bx2, nx), (by2, ny)]
            .into_iter()
            .map(|(x, n)| {
                if x < 0.0 {
                    0
                } else if x >= f64::from(n as u32) {
                    n - 1
                } else {
                    x as usize
                }
            })
            .collect();
        (out[0], out[1], out[2], out[3])
    }
}
