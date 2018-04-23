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
        // let max_axis;
        // if (major_axis.abs() - minor_axis.abs()).is_sign_positive() {
        //     max_axis = major_axis
        // } else {
        //     max_axis = minor_axis
        // }
        // Very pessimistic
        let max_axis = 0.04 + (major_axis.powi(2) + minor_axis.powi(2)).sqrt();
        let bounding_box = (
            (center_x - max_axis),
            (center_y - max_axis),
            (center_x + max_axis),
            (center_y + max_axis),
        );
        Ellipse {
            center_x,
            center_y,
            major_axis,
            minor_axis,
            theta_sin,
            theta_cos,
            intensity,
            bounding_box,
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

/// todo
pub fn shepplogan(nx: usize, ny: usize) -> Array2<f64> {
    let ellipses: [Ellipse; 10] = [
        Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.01),
        Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.01),
        Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.01),
        Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.01),
        Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.01),
        Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.01),
        Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.02),
        Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.02),
        Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98),
        Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0),
    ];
    let mut arr = Array::zeros((ny, nx));
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    arr.indexed_iter_mut()
        .map(|((y, x), a): ((usize, usize), &mut f64)| {
            let xi = (x as f64 - nx2) / nmin;
            let yi = (y as f64 - ny2) / nmin;
            *a = ellipses
                .iter()
                .filter(|e| e.inside(yi, xi))
                .map(|e| e.intensity())
                .sum();
        })
        .count();
    arr
}

/// todo
pub fn shepplogan_modified(nx: usize, ny: usize) -> Array2<f64> {
    let ellipses: [Ellipse; 10] = [
        Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.1),
        Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.1),
        Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.1),
        Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.1),
        Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.1),
        Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.1),
        Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.2),
        Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.2),
        Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.8),
        Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0),
    ];

    let mut arr = Array::zeros((ny, nx));
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    arr.indexed_iter_mut()
        .map(|((y, x), a): ((usize, usize), &mut f64)| {
            let xi = (x as f64 - nx2) / nmin;
            let yi = (y as f64 - ny2) / nmin;
            *a = ellipses
                .iter()
                .filter(|e| e.inside(yi, xi))
                .map(|e| e.intensity())
                .sum();
        })
        .count();
    arr
}

/// todo
pub fn shepplogan_modified_vec(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses: [Ellipse; 10] = [
        Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.1),
        Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.1),
        Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.1),
        Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.1),
        Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.1),
        Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.1),
        Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.2),
        Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.2),
        Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.8),
        Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0),
    ];

    let mut arr = Vec::with_capacity(nx * ny);
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    for y in 0..ny {
        for x in 0..nx {
            let xi = (x as f64 - nx2) / nmin;
            let yi = (y as f64 - ny2) / nmin;
            arr.push(
                ellipses
                    .iter()
                    .filter(|e| e.inside(yi, xi))
                    .map(|e| e.intensity())
                    .sum(),
            );
        }
    }
    arr
}

/// todo
pub fn shepplogan_modified_vec_bounding_box(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses: [Ellipse; 10] = [
        Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.1),
        Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.1),
        Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.1),
        Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.1),
        Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.1),
        Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.1),
        Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.2),
        Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.2),
        Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.8),
        Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0),
    ];

    let mut arr = vec![0.0; nx * ny];
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    for e in ellipses.iter() {
        let bbox = e.bounding_box(nx, ny);
        println!("{:?}", bbox);
        for x in bbox.1..bbox.3 {
            for y in bbox.0..bbox.2 {
                let xi = (x as f64 - nx2) / nmin;
                let yi = (y as f64 - ny2) / nmin;
                if e.inside(yi, xi) {
                    arr[y * ny + x] += e.intensity();
                }
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
