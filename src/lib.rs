// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Shepp-Logan phantom
//!
//! Have you ever had the need to create hundreds to thousands of Shepp-Logan phantoms per second?
//! Well if you do, you're doing something wrong, but you've come to the right place.
//! The Shepp-Logan phantom is a numerical phantom which is defined as the sum of 10 ellipses. It
//! is often used as a test image for image reconstruction algorithms.
//! This crate provides an efficient implementation for creating Shepp-Logan phantoms in 2D. 
//! The following results were obtained with `cargo bench` on an Intel Core i7 with 2.70GHz:
//!
//! Resolution | time        | fps   
//! -----------|-------------|------ 
//! 128x128    |   111,000ns | 9000  
//! 256x256    |   440,000ns | 2200  
//! 512x512    | 1,780,000ns |  560  
//!
//! Two versions are provided: The original version as described in [0] and a modified version,
//! which has higher contrast as described in [1]. If you do not know the difference between those
//! two, you most likely want the modified version.
//!
//! To use the crate, add `shepplogan` to your `Cargo.toml`:
//!
//! ```toml
//! shepplogan = "^1"
//! ```
//! 
//! # Example
//!
//! ```rust
//! extern crate shepplogan;
//! use shepplogan::{shepplogan, shepplogan_modified};
//!
//! // Dimensions of the image grid
//! let (nx, ny) = (256, 320);
//!
//! // Original Shepp-Logan Phantom (the dynamic range is between 0.0 and 2.0)
//! let phantom = shepplogan(nx, ny);
//!
//! // Modified Shepp-Logan Phantom (the dynamic range is between 0.0 and 1.0)
//! let phantom_modified = shepplogan_modified(nx, ny);
//! ```
//!
//! See `examples/example.rs` for an example which saves the phantom to disk.
//!
//! You can also create your own phantom by defining ellipses:
//!
//! ```rust
//! extern crate shepplogan;
//! use shepplogan::{phantom, Ellipse};
//!
//! // Dimensions of the image grid
//! let (nx, ny) = (256, 320);
//!
//! // Define two ellipses
//! let ellipses = 
//!     [
//!         Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98),
//!         Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 2.0),
//!     ];
//!
//! let ph = phantom(&ellipses, nx, ny);
//! ```
//! 
//! This will create a phantom consisting of two ellipses.
//!
//! # References
//!
//! [0] Shepp, LA and Logan BF, "The Fourier reconstruction of a head section." IEEE Transactions
//! on Nuclear Science 21, No. 3 (1974)
//!
//! [1] Toft, PA, "The Radon Transform - Theory and Implementation", PhD dissertation, Departement
//! of Mathematical Modelling, Technical University of Denmark (1996)

#![warn(missing_docs)]
#[cfg(feature = "parallel")]
extern crate rayon;

mod ellipse;
pub use ellipse::Ellipse;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "parallel")]
use std::sync::Mutex;

macro_rules! parts {
    () => {
        [
            Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.01),
            Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.01),
            Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.01),
            Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.01),
            Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.01),
            Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.01),
            Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.02),
            Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.02),
            Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98),
            Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 2.0),
        ]
    };
}

macro_rules! parts_modified {
    () => {
        [
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
        ]
    };
}

/// Original Shepp-Logan phantom
///
/// Constructs the original Shepp-Logan phantom as described in:
///
/// Shepp, LA and Logan BF, "The Fourier reconstruction of a head section." IEEE Transactions on
/// Nuclear Science 21, No. 3 (1974)
///
/// The parameters `nx` and `ny` define the number of pixels in `x` and `y` direction.
/// The dynamic range of the values is between `0.0` and `2.0`.
pub fn shepplogan(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts!();
    phantom(&ellipses, nx, ny)
}

/// Modified Shepp-Logan phantom with better contrast
///
/// Constructs the modified Shepp-Logan phantom as described in:
///
/// Toft, PA, "The Radon Transform - Theory and Implementation", PhD dissertation, Departement of
/// Mathematical Modelling, Technical University of Denmark (1996)
///
/// The parameters `nx` and `ny` define the number of pixels in `x` and `y` direction.
/// The dynamic range of the values is between `0.0` and `1.0`.
pub fn shepplogan_modified(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts_modified!();
    phantom(&ellipses, nx, ny)
}

#[cfg(not(feature = "parallel"))]
#[cfg(not(feature = "slow_impl"))]
/// Creates a phantom based on given ellipses
///
/// Besides `nx` and `ny`, which define the number of pixels in `x` and `y` direction, this
/// function also requires a vector of Ellipses.
pub fn phantom(ellipses: &[Ellipse], nx: usize, ny: usize) -> Vec<f64> {
    let mut arr = vec![0.0; nx * ny];
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    for e in ellipses.iter() {
        let bbox = e.bounding_box(nx, ny);
        for x in bbox.0..bbox.2 {
            let xi = (x as f64 - nx2) / nmin;
            for y in bbox.1..bbox.3 {
                let yi = (y as f64 - ny2) / nmin;
                if e.inside(xi, yi) {
                    arr[(ny-y) * nx + x] += e.intensity();
                }
            }
        }
    }
    arr
}

#[cfg(feature = "parallel")]
pub fn phantom(ellipses: &[Ellipse], nx: usize, ny: usize) -> Vec<f64> {
    let arr: Vec<Mutex<f64>> = (0..(nx * ny))
        .into_par_iter()
        .map(|_| Mutex::new(0.0))
        .collect();
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    ellipses.into_par_iter().for_each(|e| {
        let bbox = e.bounding_box(nx, ny);
        (bbox.0..bbox.2).into_iter().for_each(|x| {
            let xi = (x as f64 - nx2) / nmin;
            (bbox.1..bbox.3).into_iter().for_each(|y| {
                let yi = (y as f64 - ny2) / nmin;
                if e.inside(xi, yi) {
                    let mut b = arr[(ny - y) * nx + x].lock().unwrap();
                    *b = *b + e.intensity();
                }
            })
        });
    });
    arr.into_par_iter().map(|x| *(x.lock().unwrap())).collect()
}

#[cfg(feature = "slow_impl")]
pub fn phantom(ellipses: &[Ellipse], nx: usize, ny: usize) -> Vec<f64> {
    let mut arr = Vec::with_capacity(nx * ny);
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    for y in 0..ny {
        for x in 0..nx {
            let xi = (x as f64 - nx2) / nmin;
            let yi = ((ny-y) as f64 - ny2) / nmin;
            arr.push(
                ellipses
                    .iter()
                    .filter(|e| e.inside(xi, yi))
                    .map(|e| e.intensity())
                    .sum(),
            );
        }
    }
    arr
}
