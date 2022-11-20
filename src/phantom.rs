// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{shape::ShapeOnCanvas, Shape};

/// General phantom
///
/// todo
pub struct Phantom {
    data: Vec<f64>,
    minmax: Option<(f64, f64)>,
}

impl Phantom {
    /// Create a new phantom with size `nx` times `ny` given a set of `ellipses`.
    pub fn new(nx: u32, ny: u32, shapes: &[Shape]) -> Self {
        let shapes = shapes
            .iter()
            .map(|shape| shape.on_canvas(nx, ny))
            .collect::<Vec<_>>();
        let data = phantom(&shapes, nx, ny);
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

    /// Returns the phantom as a flattened `Vec<U>`. where `U: From<f64>`.
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
/// function also requires array of ShapeOnCanvas.
fn phantom(shapes: &[ShapeOnCanvas], nx: u32, ny: u32) -> Vec<f64> {
    let mut arr = vec![0.0; (nx * ny) as usize];

    for shape in shapes.iter() {
        let bbox = shape.bounding_box();
        for x in bbox.x_low..=bbox.x_high {
            let xi = f64::from(x);
            for y in bbox.y_low..=bbox.y_high {
                let yi = f64::from(y);
                if shape.inside(xi, yi) {
                    arr[((ny - y - 1) * nx + x) as usize] += shape.intensity();
                }
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use crate::Shape;

    use super::phantom;

    #[derive(Debug, Copy, Clone)]
    struct FloatNotNanSmall(f64);

    impl quickcheck::Arbitrary for FloatNotNanSmall {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let val = f64::arbitrary(g).abs() % 10.0;
                if !val.is_nan() && val.is_finite() {
                    return FloatNotNanSmall(val);
                }
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct UnsignedInt32(u32);

    impl quickcheck::Arbitrary for UnsignedInt32 {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            UnsignedInt32(u32::arbitrary(g) % 128)
        }
    }

    #[quickcheck]
    // Add a reason why this lint is allowed once the feature `lint_reasons` is stabilized.
    #[allow(clippy::too_many_arguments)]
    fn test_phantom(
        center_x: FloatNotNanSmall,
        center_y: FloatNotNanSmall,
        major_axis: FloatNotNanSmall,
        minor_axis: FloatNotNanSmall,
        theta: FloatNotNanSmall,
        nx: UnsignedInt32,
        ny: UnsignedInt32,
    ) {
        let nx = nx.0;
        let ny = ny.0;
        let center_x = center_x.0;
        let center_y = center_y.0;
        let major_axis = major_axis.0;
        let minor_axis = minor_axis.0;
        let theta = theta.0;

        let shape = Shape::ellipse(center_x, center_y, major_axis, minor_axis, theta, 1.0)
            .on_canvas(nx, ny);

        let phantom = phantom(&[shape.clone()], nx, ny);

        for x in 0..nx {
            for y in 0..ny {
                let val = phantom[((ny - y - 1) * nx + x) as usize];
                let inside = shape.inside(f64::from(x), f64::from(y));
                // println!("x: {} | y: {} | val: {} | inside: {}", x, y, val, inside);
                if inside {
                    assert_eq!(val.to_ne_bytes(), 1.0f64.to_ne_bytes());
                } else {
                    assert_eq!(val.to_ne_bytes(), 0.0f64.to_ne_bytes());
                }
            }
        }
    }
}
