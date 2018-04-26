// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(missing_docs)]
#[cfg(feature = "parallel")]
extern crate mucow;
#[cfg(feature = "parallel")]
extern crate rayon;

mod ellipse;
use ellipse::Ellipse;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
// #[cfg(feature = "parallel")]
// use std::sync::Arc;
#[cfg(feature = "parallel")]
use std::sync::Mutex;
// #[cfg(feature = "parallel")]
// use std::sync::RwLock;

macro_rules! parts {
    () => {
        [
            Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.01),
            Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.01),
            Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.01),
            Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.01),
            Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.01),
            Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.01),
            Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.02),
            Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.02),
            Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98),
            Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0),
        ]
    };
}

macro_rules! parts_modified {
    () => {
        [
            Ellipse::new(0.0, 0.35, 0.21, 0.25, 0.0, 0.1),
            Ellipse::new(0.0, 0.1, 0.046, 0.046, 0.0, 0.1),
            Ellipse::new(0.0, -0.1, 0.046, 0.046, 0.0, 0.1),
            Ellipse::new(-0.08, -0.605, 0.046, 0.023, 0.0, 0.1),
            Ellipse::new(0.0, -0.605, 0.023, 0.023, 0.0, 0.1),
            Ellipse::new(0.06, -0.605, 0.023, 0.046, 0.0, 0.1),
            Ellipse::new(0.22, 0.0, 0.11, 0.31, -18.0, -0.2),
            Ellipse::new(-0.22, 0.0, 0.16, 0.41, 18.0, -0.2),
            Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.8),
            Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 1.0),
        ]
    };
}

/// todo
pub fn shepplogan_slow(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts!();
    phantom_slow(&ellipses, nx, ny)
}

/// todo
#[cfg(not(feature = "parallel"))]
pub fn shepplogan(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts!();
    phantom(&ellipses, nx, ny)
}

/// todo
#[cfg(feature = "parallel")]
pub fn shepplogan(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts!();
    phantom_parallel(&ellipses, nx, ny)
}

/// todo
pub fn shepplogan_modified_slow(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts_modified!();
    phantom_slow(&ellipses, nx, ny)
}

/// todo
#[cfg(not(feature = "parallel"))]
pub fn shepplogan_modified(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts_modified!();
    phantom(&ellipses, nx, ny)
}

/// todo
#[cfg(feature = "parallel")]
pub fn shepplogan_modified(nx: usize, ny: usize) -> Vec<f64> {
    let ellipses = parts_modified!();
    phantom_parallel(&ellipses, nx, ny)
}

/// todo
#[cfg(not(feature = "parallel"))]
fn phantom(ellipses: &[Ellipse], nx: usize, ny: usize) -> Vec<f64> {
    let mut arr = vec![0.0; nx * ny];
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    for e in ellipses.iter() {
        let bbox = e.bounding_box(nx, ny);
        for x in bbox.1..bbox.3 {
            for y in bbox.0..bbox.2 {
                let xi = (x as f64 - nx2) / nmin;
                let yi = (y as f64 - ny2) / nmin;
                if e.inside(yi, xi) {
                    arr[y * ny + x] += e.intensity();
                }
            }
        }
    }
    arr
}

/// todo
#[cfg(feature = "parallel")]
fn phantom_parallel(ellipses: &[Ellipse], nx: usize, ny: usize) -> Vec<f64> {
    let arr: Vec<Mutex<f64>> = (0..(nx * ny))
        .into_par_iter()
        .map(|_| Mutex::new(0.0))
        .collect();
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    ellipses.into_par_iter().for_each(|e| {
        let bbox = e.bounding_box(nx, ny);
        (bbox.1..bbox.3).into_par_iter().for_each(|x| {
            let xi = (x as f64 - nx2) / nmin;
            (bbox.0..bbox.2).into_par_iter().for_each(|y| {
                let yi = (y as f64 - ny2) / nmin;
                if e.inside(yi, xi) {
                    let mut b = arr[y * ny + x].lock().unwrap();
                    *b = *b + e.intensity();
                }
            })
        });
    });
    arr.into_par_iter().map(|x| *(x.lock().unwrap())).collect()
}

fn phantom_slow(ellipses: &[Ellipse], nx: usize, ny: usize) -> Vec<f64> {
    let mut arr = Vec::with_capacity(nx * ny);
    let nx2 = (nx as f64) / 2.0;
    let ny2 = (ny as f64) / 2.0;
    let nmin = (std::cmp::min(nx, ny) as f64) / 2.0;

    for y in 0..ny {
        for x in 0..nx {
            let xi = (x as f64 - nx2) / nmin;
            let yi = (y as f64 - ny2) / nmin;
            arr.push(
                ellipses
                    .iter()
                    .filter(|e| e.inside(yi, xi))
                    .map(|e| e.intensity())
                    .sum(),
            );
        }
    }
    arr
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
