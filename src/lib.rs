// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(missing_docs)]

extern crate ndarray;
use ndarray::prelude::*;
use ndarray::Array2;
// use ndarray::{arr1, arr2};

/// ellipse
struct Ellipse {
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
        Ellipse {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            theta_sin,
            theta_cos,
            intensity,
        }
    }

    /// todo
    /// TODO: x and y is inconsistent...
    pub fn inside(&self, x: f64, y: f64) -> bool {
        (self.theta_cos * (x - self.center_x) + self.theta_sin * (y - self.center_y)).powi(2)
            / self.major_axis.powi(2)
            + (self.theta_sin * (x - self.center_x) + self.theta_cos * (y - self.center_y)).powi(2)
                / self.minor_axis.powi(2) <= 1.0
    }

    /// todo
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
}

/// todo
pub fn shepplogan(nx: usize, ny: usize) -> Array2<f64> {
    let mut ellipses: Vec<Ellipse> = Vec::with_capacity(10);
    ellipses.push(Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0));
    ellipses.push(Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98));
    ellipses.push(Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.02));
    ellipses.push(Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.02));
    ellipses.push(Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.01));
    ellipses.push(Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.01));
    ellipses.push(Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.01));
    ellipses.push(Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.01));
    ellipses.push(Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.01));
    ellipses.push(Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.01));
    let mut arr = Array::zeros((ny, nx));
    arr.indexed_iter_mut()
        .map(|((y, x), a): ((usize, usize), &mut f64)| {
            let nx2 = (nx as f64) / 2.0;
            let ny2 = (ny as f64) / 2.0;
            let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;
            let x = (x as f64 - nx2) / nmin;
            let y = (y as f64 - ny2) / nmin;
            let mut f = 0.0;
            for e in ellipses.iter() {
                if e.inside(y, x) {
                    f += e.intensity();
                    // *a += e.intensity();
                }
            }
            *a += f;
            // ellipses
            //     .iter()
            //     .filter(move |&b| b.inside(x, y))
            //     .map(|b| *a += b.intensity())
        })
        .count();
    arr
}

/// todo
pub fn shepplogan_modified(nx: usize, ny: usize) -> Array2<f64> {
    let mut ellipses: Vec<Ellipse> = Vec::with_capacity(10);
    ellipses.push(Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0));
    ellipses.push(Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.8));
    ellipses.push(Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.2));
    ellipses.push(Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.2));
    ellipses.push(Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.1));
    ellipses.push(Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.1));
    ellipses.push(Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.1));
    ellipses.push(Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.1));
    ellipses.push(Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.1));
    ellipses.push(Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.1));
    let mut arr = Array::zeros((ny, nx));
    arr.indexed_iter_mut()
        .map(|((y, x), a): ((usize, usize), &mut f64)| {
            let nx2 = (nx as f64) / 2.0;
            let ny2 = (ny as f64) / 2.0;
            let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;
            let x = (x as f64 - nx2) / nmin;
            let y = (y as f64 - ny2) / nmin;
            let mut f = 0.0;
            for e in ellipses.iter() {
                if e.inside(y, x) {
                    f += e.intensity();
                    // *a += e.intensity();
                }
            }
            *a += f;
        })
        .count();
    arr
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
