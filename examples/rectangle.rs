// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use shepplogan::{Phantom, Shape};

fn main() {
    let nx = 512;
    let ny = 640;
    // // let ny = 256;
    // let nx = 1024;
    // let ny = 1280;

    let num_rectangles: u32 = 5;

    let step = 90.0 / f64::from(num_rectangles);

    let mut shapes: Vec<_> = (0..num_rectangles)
        .map(|s| {
            Shape::rectangle(
                0.0,
                0.0,
                1.0,
                1.0,
                f64::from(s) * step,
                255.0 / f64::from(num_rectangles),
            )
        })
        .collect();

    let mut shapes2: Vec<_> = (0..num_rectangles)
        .map(|s| {
            Shape::rectangle(
                0.0,
                0.0,
                0.6,
                0.6,
                f64::from(s) * step,
                -255.0 / f64::from(num_rectangles),
            )
        })
        .collect();

    shapes.append(&mut shapes2);

    let phantom = Phantom::new(nx, ny, &shapes).into_vec_u8();

    image::save_buffer("rectangle.png", &phantom, nx, ny, image::ColorType::L8).unwrap();
}
