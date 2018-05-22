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
    /// major axis
    major_axis: f64,
    /// minor axis
    minor_axis: f64,
    /// sin(theta)
    theta_sin: f64,
    /// cos(theta)
    theta_cos: f64,
    /// intensity
    intensity: f64,
    /// bounding box
    #[cfg(not(feature = "slow_impl"))]
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
        #[cfg(not(feature = "slow_impl"))]
        let bbx = ((theta_cos * major_axis).powi(2) + (theta_sin * minor_axis).powi(2)).sqrt();
        #[cfg(not(feature = "slow_impl"))]
        let bby = ((theta_sin * major_axis).powi(2) + (theta_cos * minor_axis).powi(2)).sqrt();
        #[cfg(not(feature = "slow_impl"))]
        let bounding_box = (
            (center_x - bbx),
            (center_y - bby),
            (center_x + bbx),
            (center_y + bby),
        );
        Ellipse {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            theta_sin,
            theta_cos,
            intensity,
            #[cfg(not(feature = "slow_impl"))]
            bounding_box,
        }
    }

    /// Checks if a point is inside the ellipse
    pub fn inside(&self, x: f64, y: f64) -> bool {
        (self.theta_cos * (x - self.center_x) + self.theta_sin * (y - self.center_y)).powi(2)
            / self.major_axis.powi(2)
            + (self.theta_sin * (x - self.center_x) - self.theta_cos * (y - self.center_y)).powi(2)
                / self.minor_axis.powi(2) <= 1.0
    }

    /// Return intensity of the ellipse
    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    #[cfg(not(feature = "slow_impl"))]
    /// Return the bounding box of the ellipse
    pub fn bounding_box(&self, nx: usize, ny: usize) -> (usize, usize, usize, usize) {
        let bx1 = ((self.bounding_box.0 + 1.0) * (nx as f64) / 2.0).floor();
        let by1 = ((self.bounding_box.1 + 1.0) * (ny as f64) / 2.0).floor();
        let bx2 = ((self.bounding_box.2 + 1.0) * (nx as f64) / 2.0).ceil();
        let by2 = ((self.bounding_box.3 + 1.0) * (ny as f64) / 2.0).ceil();
        let out: Vec<usize> = [bx1, by1, bx2, by2]
            .iter()
            .zip([nx, ny, nx, ny].iter())
            .map(|(x, n)| {
                if *x < 0.0 {
                    0
                } else if *x > *n as f64 {
                    *n
                } else {
                    *x as usize
                }
            })
            .collect();
        (out[0], out[1], out[2], out[3])
    }
}
