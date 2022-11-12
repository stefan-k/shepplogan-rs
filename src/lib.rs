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
//! This crate provides a dependency-free, efficient implementation for creating Shepp-Logan
//! phantoms in 2D.
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

mod ellipse;
pub use crate::ellipse::Ellipse;

/// Original Shepp-Logan phantom
///
/// Constructs the original Shepp-Logan phantom as described in:
///
/// Shepp, LA and Logan BF, "The Fourier reconstruction of a head section." IEEE Transactions on
/// Nuclear Science 21, No. 3 (1974)
///
/// The parameters `nx` and `ny` define the number of pixels in `x` and `y` direction.
/// The dynamic range of the values is between `0.0` and `2.0`.
pub fn shepplogan(nx: u32, ny: u32) -> Vec<f64> {
    let ellipses = [
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
    ];
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
pub fn shepplogan_modified(nx: u32, ny: u32) -> Vec<f64> {
    let ellipses = [
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
    phantom(&ellipses, nx, ny)
}

/// Creates a phantom based on given ellipses
///
/// Besides `nx` and `ny`, which define the number of pixels in `x` and `y` direction, this
/// function also requires a vector of Ellipses.
pub fn phantom(ellipses: &[Ellipse], nx: u32, ny: u32) -> Vec<f64> {
    let mut arr = vec![0.0; (nx * ny) as usize];
    let nx2 = f64::from(nx) / 2.0;
    let ny2 = f64::from(ny) / 2.0;
    let nmin = f64::from(std::cmp::min(nx, ny)) / 2.0;

    for e in ellipses.iter() {
        let bbox = e.bounding_box(nx, ny);
        for x in bbox.0..bbox.2 {
            let xi = (x as f64 - nx2) / nmin;
            for y in bbox.1..bbox.3 {
                // if x == bbox.0 || x == bbox.2 - 1 || y == bbox.1 || y == bbox.3 - 1 {
                //     arr[((ny - y - 1) * nx + x - 1) as usize] = 1.0;
                // }
                let yi = (y as f64 - ny2) / nmin;

                // arr[((ny - y - 1) * nx + x - 1) as usize] += e.inside_2(xi, yi);
                if e.inside(xi, yi) {
                    arr[((ny - y - 1) * nx + x - 1) as usize] += e.intensity();
                }
            }
        }
    }
    arr
}

/// TODOOOOO
pub struct SheppLogan<const M: usize, const N: usize> {
    data: Box<[[f64; M]; N]>,
}

impl<const M: usize, const N: usize> SheppLogan<M, N> {
    /// todoooo
    pub fn new() -> SheppLogan<M, N> {
        let mut data: Box<[[f64; M]; N]> = Box::new([[0.0; M]; N]);
        let ellipses = [
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
        let nx2 = f64::from(N as u32) / 2.0;
        let ny2 = f64::from(M as u32) / 2.0;
        let nmin = f64::from(std::cmp::min(N, M) as u32) / 2.0;

        for e in ellipses.iter() {
            let bbox = e.bounding_box_usize(N, M);
            for y in bbox.1..bbox.3 {
                let yi = (y as f64 - ny2) / nmin;
                for x in bbox.0..bbox.2 {
                    // if x == bbox.0 || x == bbox.2 - 1 || y == bbox.1 || y == bbox.3 - 1 {
                    //     data[y as usize][x as usize] = 1.0;
                    // }
                    let xi = (x as f64 - nx2) / nmin;
                    data[y][x] += e.inside_2(xi, yi);
                    // data[y][x] += if e.inside(xi, yi) { e.intensity() } else { 0.0 };
                    // if e.inside(xi, yi) {
                    //     data[y][x] += e.intensity();
                    // }
                }
            }
        }
        SheppLogan { data }
    }

    /// todo
    pub fn to_vec(&self) -> Vec<f64> {
        self.data
            .iter()
            .rev()
            .flat_map(|v| v.iter())
            .cloned()
            .collect()
    }
}
