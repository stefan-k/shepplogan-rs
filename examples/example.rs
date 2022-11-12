// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use shepplogan::{Phantom, SheppLogan};

fn main() {
    // let nx = 256;
    // let ny = 320;
    // // let ny = 256;
    let nx = 1024;
    let ny = 1280;
    // let ny = 1024;

    // // Original Shepp-Logan phantom
    let phantom = shepplogan::shepplogan(nx, ny);

    let phantom: Vec<u8> = phantom.iter().map(|x| (*x / 2.0 * 255.0) as u8).collect();
    image::save_buffer(
        "shepp_logan.png",
        &phantom,
        nx as u32,
        ny as u32,
        image::ColorType::L8,
    )
    .unwrap();

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

    let phantom: Vec<u8> = SheppLogan::new(nx, ny, true).scale(255.0).into_vec();
    image::save_buffer(
        "shepp_logan_modified_new.png",
        &phantom,
        nx as u32,
        ny as u32,
        image::ColorType::L8,
    )
    .unwrap();
}
