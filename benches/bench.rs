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
    use shepplogan::{shepplogan, shepplogan_modified, shepplogan_modified_vec,
                     shepplogan_modified_vec_bounding_box};
    use test::{black_box, Bencher};

    #[bench]
    fn shepplogan_256(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan(256, 256));
        });
    }

    #[bench]
    fn shepplogan_128(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan(128, 128));
        });
    }

    #[bench]
    fn shepplogan_512(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan(512, 512));
        });
    }

    #[bench]
    fn shepplogan_modified_256(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified(256, 256));
        });
    }

    #[bench]
    fn shepplogan_modified_128(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified(128, 128));
        });
    }

    #[bench]
    fn shepplogan_modified_512(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified(512, 512));
        });
    }

    #[bench]
    fn shepplogan_modified_vec_256(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_vec(256, 256));
        });
    }

    #[bench]
    fn shepplogan_modified_vec_128(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_vec(128, 128));
        });
    }

    #[bench]
    fn shepplogan_modified_vec_512(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_vec(512, 512));
        });
    }

    #[bench]
    fn shepplogan_modified_vec_bb_256(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_vec_bounding_box(256, 256));
        });
    }

    #[bench]
    fn shepplogan_modified_vec_bb_128(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_vec_bounding_box(128, 128));
        });
    }

    #[bench]
    fn shepplogan_modified_vec_bb_512(b: &mut Bencher) {
        b.iter(|| {
            black_box(shepplogan_modified_vec_bounding_box(512, 512));
        });
    }
}
