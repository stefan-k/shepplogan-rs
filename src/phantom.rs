// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Shape;

/// General phantom
///
/// todo
pub struct Phantom {
    data: Vec<f64>,
    minmax: Option<(f64, f64)>,
}

impl Phantom {
    /// Create a new phantom with size `nx` times `ny` given a set of `ellipses`.
    pub fn new(nx: u32, ny: u32, ellipses: &[Shape]) -> Self {
        let data = phantom(ellipses, nx, ny);
        Phantom { data, minmax: None }
    }

    /// Scales the value of the phantom with `factor`.
    pub fn scale(mut self, factor: f64) -> Phantom {
        self.data = self.data.into_iter().map(|x| x * factor).collect();
        self.minmax = if let Some((min, max)) = self.minmax {
            Some((min * factor, max * factor))
        } else {
            None
        };
        self
    }

    /// Returns the minimum and maximum value of the phantom as `(min, max)`.
    ///
    /// This function takes `&mut self` because once minium and maximum are calculated, they values
    /// are cached internally to avoid recomputation when calling this function multiple times.
    pub fn extrema(&mut self) -> (f64, f64) {
        if let Some(minmax) = self.minmax {
            minmax
        } else {
            let minmax = self.data.iter().fold(
                (f64::INFINITY, f64::NEG_INFINITY),
                |(acc_min, acc_max), &x| {
                    (
                        if x < acc_min { x } else { acc_min },
                        if x > acc_max { x } else { acc_max },
                    )
                },
            );
            self.minmax = Some(minmax);
            minmax
        }
    }

    /// Returns the phantom as a flattened `Vec<f64>`.
    pub fn into_vec<U: From<f64>>(self) -> Vec<U> {
        self.data.into_iter().map(|x| U::from(x)).collect()
    }

    /// Returns the phantom as a `Vec<u8>`
    ///
    /// Note that this will cast `f64` to `u8`, therefore the caller must ensure that the current
    /// `f64` values of the phantom are within the range `[0, 255)`.
    pub fn into_vec_u8(self) -> Vec<u8> {
        self.data.into_iter().map(|x| x as u8).collect()
    }
}

/// Creates a phantom based on given ellipses
///
/// Besides `nx` and `ny`, which define the number of pixels in `x` and `y` direction, this
/// function also requires a vector of Shapes.
fn phantom(ellipses: &[Shape], nx: u32, ny: u32) -> Vec<f64> {
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
                if e.inside(xi, yi) {
                    arr[((ny - y - 1) * nx + x - 1) as usize] += e.intensity();
                }
            }
        }
    }
    arr
}
