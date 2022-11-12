// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use shepplogan::SheppLogan;

extern crate image;
extern crate shepplogan;

fn main() {
    // let nx = 256;
    // let ny = 320;
    // // let ny = 256;
    // let nx = 1024;
    // let ny = 1280;
    // let nx = 128;
    // let ny = 128;
    // let nx = 256;
    // let ny = 256;
    let nx = 512;
    let ny = 512;
    // let ny = 1024;

    // // Original Shepp-Logan phantom
    // let phantom = shepplogan::shepplogan(nx, ny);

    // let phantom: Vec<u8> = phantom.iter().map(|x| (*x / 2.0 * 255.0) as u8).collect();
    // image::save_buffer(
    //     "shepp_logan.png",
    //     &phantom,
    //     nx as u32,
    //     ny as u32,
    //     image::ColorType::L8,
    // )
    // .unwrap();

    // Modified Shepp-Logan phantom
    let phantom = shepplogan::shepplogan_modified(nx, ny);

    let phantom: Vec<u8> = phantom.iter().map(|x| (*x * 255.0) as u8).collect();
    image::save_buffer(
        "shepp_logan_modified.png",
        &phantom,
        nx as u32,
        ny as u32,
        image::ColorType::L8,
    )
    .unwrap();

    let phantom: SheppLogan<512, 512> = SheppLogan::new();
    let phantom = phantom.to_vec();

    let phantom: Vec<u8> = phantom.iter().map(|x| (*x * 255.0) as u8).collect();
    image::save_buffer(
        "shepp_logan_modified_const.png",
        &phantom,
        512,
        512,
        image::ColorType::L8,
    )
    .unwrap();
}
