// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{Phantom, Shape};

/// Original Shepp-Logan phantom
///
/// Constructs the original Shepp-Logan phantom as described in:
///
/// Shepp, LA and Logan BF, "The Fourier reconstruction of a head section." IEEE Transactions on
/// Nuclear Science 21, No. 3 (1974)
///
/// The parameters `nx` and `ny` define the number of pixels in `x` and `y` direction.
/// The dynamic range of the values is between `0.0` and `2.0`.
pub fn shepplogan(nx: u32, ny: u32) -> Phantom {
    let ellipses = [
        Shape::ellipse(0.0, 0.35, 0.21, 0.25, 0.0, 0.01),
        Shape::ellipse(0.0, 0.1, 0.046, 0.046, 0.0, 0.01),
        Shape::ellipse(0.0, -0.1, 0.046, 0.046, 0.0, 0.01),
        Shape::ellipse(-0.08, -0.605, 0.046, 0.023, 0.0, 0.01),
        Shape::ellipse(0.0, -0.605, 0.023, 0.023, 0.0, 0.01),
        Shape::ellipse(0.06, -0.605, 0.023, 0.046, 0.0, 0.01),
        Shape::ellipse(0.22, 0.0, 0.11, 0.31, -18.0, -0.02),
        Shape::ellipse(-0.22, 0.0, 0.16, 0.41, 18.0, -0.02),
        Shape::ellipse(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98),
        Shape::ellipse(0.0, 0.0, 0.69, 0.92, 0.0, 2.0),
    ];
    Phantom::new(nx, ny, &ellipses)
}

/// Modified Shepp-Logan phantom with increased contrast
///
/// Constructs the modified Shepp-Logan phantom as described in:
///
/// Toft, PA, "The Radon Transform - Theory and Implementation", PhD dissertation, Departement of
/// Mathematical Modelling, Technical University of Denmark (1996)
///
/// The parameters `nx` and `ny` define the number of pixels in `x` and `y` direction.
/// The dynamic range of the values is between `0.0` and `1.0`.
pub fn shepplogan_modified(nx: u32, ny: u32) -> Phantom {
    let ellipses = [
        Shape::ellipse(0.0, 0.35, 0.21, 0.25, 0.0, 0.1),
        Shape::ellipse(0.0, 0.1, 0.046, 0.046, 0.0, 0.1),
        Shape::ellipse(0.0, -0.1, 0.046, 0.046, 0.0, 0.1),
        Shape::ellipse(-0.08, -0.605, 0.046, 0.023, 0.0, 0.1),
        Shape::ellipse(0.0, -0.605, 0.023, 0.023, 0.0, 0.1),
        Shape::ellipse(0.06, -0.605, 0.023, 0.046, 0.0, 0.1),
        Shape::ellipse(0.22, 0.0, 0.11, 0.31, -18.0, -0.2),
        Shape::ellipse(-0.22, 0.0, 0.16, 0.41, 18.0, -0.2),
        Shape::ellipse(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.8),
        Shape::ellipse(0.0, 0.0, 0.69, 0.92, 0.0, 1.0),
    ];
    Phantom::new(nx, ny, &ellipses)
}
