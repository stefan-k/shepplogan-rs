// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate image;
// extern crate ndarray;
extern crate shepplogan;

// use ndarray::prelude::*;
use image::{GenericImage, ImageBuffer};

fn main() {
    let nx = 256;
    let mut bla = shepplogan::shepplogan(nx, nx);
    // println!("{:#?}", bla);

    let bla = bla.into_raw_vec();
    // let bla: Vec<u8> = bla.iter().map(|x| (x * 255.0 / 2.0) as u8).collect();
    // let bla: Vec<u8> = bla.iter().map(|x| (*x * 255.0 / 2.0) as u8).collect();
    let bla: Vec<u8> = bla.iter().map(|x| (*x * 255.0 / 2.0) as u8).collect();
    // println!("{:#?}", bla);
    image::save_buffer("image.bmp", &bla, nx as u32, nx as u32, image::Gray(8));
    // let img = ImageBuffer::new(nx, nx);
    // let img =
    //     ImageBuffer::<image::Luma, Vec<f64>>::from_raw(nx as u32, nx as u32, bla.into_raw_vec());
}
