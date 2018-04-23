// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO

#![feature(test)]
#![feature(concat_idents)]

extern crate shepplogan;
extern crate test;

#[cfg(test)]
mod tests {
    use shepplogan::{shepplogan, shepplogan_modified, shepplogan_modified_slow, shepplogan_slow};
    use test::{black_box, Bencher};

    // 128x128
    #[bench]
    fn shepplogan_128_slow(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_slow(128, 128));
        });
    }

    #[bench]
    fn shepplogan_128_modified_slow(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_slow(128, 128));
        });
    }

    #[bench]
    fn shepplogan_128(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan(128, 128));
        });
    }

    #[bench]
    fn shepplogan_128_modified(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified(128, 128));
        });
    }

    // 256x256
    #[bench]
    fn shepplogan_256_slow(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_slow(256, 256));
        });
    }

    #[bench]
    fn shepplogan_256_modified_slow(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_slow(256, 256));
        });
    }

    #[bench]
    fn shepplogan_256(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan(256, 256));
        });
    }

    #[bench]
    fn shepplogan_256_modified(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified(256, 256));
        });
    }

    // 512x512
    #[bench]
    fn shepplogan_512_slow(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_slow(512, 512));
        });
    }

    #[bench]
    fn shepplogan_512_modified_slow(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_slow(512, 512));
        });
    }

    #[bench]
    fn shepplogan_512(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan(512, 512));
        });
    }

    #[bench]
    fn shepplogan_512_modified(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified(512, 512));
        });
    }
}
