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
//! use shepplogan::{Phantom, Shape};
//!
//! // Dimensions of the image grid
//! let (nx, ny) = (256, 320);
//!
//! // Define two ellipses
//! let ellipses =
//!     [
//!         Shape::ellipse(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98),
//!         Shape::ellipse(0.0, 0.0, 0.69, 0.92, 0.0, 2.0),
//!     ];
//!
//! let ph = Phantom::new(nx, ny, &ellipses);
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

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod phantom;
mod shape;
mod shepplogan;

pub use crate::{
    phantom::Phantom,
    shape::Shape,
    shepplogan::{shepplogan, shepplogan_modified},
};
