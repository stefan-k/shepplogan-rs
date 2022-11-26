<p align="center">
    shepplogan-rs
</p>

<p align="center">
  <a href="https://docs.rs/shepplogan-rs">Docs (latest release)</a>
  |
  <a href="https://stefan-k.github.io/shepplogan-rs/shepplogan-rs/">Docs (main branch, TODO)</a>
  |
  <a href="https://github.com/stefan-k/shepplogan-rs/tree/main/examples">Examples (latest release)</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/shepplogan"
    ><img
      src="https://img.shields.io/crates/v/shepplogan?style=flat-square"
      alt="Crates.io version"
  /></a>
  <a href="https://crates.io/crates/shepplogan"
    ><img
      src="https://img.shields.io/crates/d/shepplogan?style=flat-square"
      alt="Crates.io downloads"
  /></a>
  <a href="https://github.com/stefan-k/shepplogan-rs/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/stefan-k/shepplogan-rs/shepplogan/main?label=shepplogan CI&style=flat-square"
      alt="GitHub Actions workflow status"
  /></a>
  <a href="https://app.codecov.io/gh/stefan-k/shepplogan-rs/"
    ><img
      src="https://img.shields.io/codecov/c/github/stefan-k/shepplogan-rs?style=flat-square"
      alt="Code coverage"
  /></a>
  <img
    src="https://img.shields.io/crates/l/shepplogan?style=flat-square"
    alt="License"
  />
</p>


# shepplogan-rs

Have you ever had the need to create hundreds to thousands of Shepp-Logan phantoms per second?
Well if you do, you're doing something wrong, but you've come to the right place.
The Shepp-Logan phantom is a numerical phantom which is defined as the sum of 10 ellipses. It
is often used as a test image for image reconstruction algorithms.
This crate provides a dependency-free, efficient implementation for creating Shepp-Logan
phantoms in 2D. 
The following results were obtained with `cargo bench` on an Intel Core i7 with 2.70GHz:

Resolution | time        | fps   
-----------|-------------|------ 
128x128    |   111,000ns | 9000  
256x256    |   440,000ns | 2200  
512x512    | 1,780,000ns |  560  

Two versions are provided: The original version as described in [0] and a modified version,
which has higher contrast as described in [1]. If you do not know the difference between those
two, you most likely want the modified version.

To use the crate, add `shepplogan` to your `Cargo.toml`:

```toml
shepplogan = "^1"
```

The documentation can be found [here](https://stefan-k.github.io/shepplogan-rs/shepplogan/).
 
## Example

```rust
extern crate shepplogan;
use shepplogan::{shepplogan, shepplogan_modified};

// Dimensions of the image grid
let (nx, ny) = (256, 320);

// Original Shepp-Logan Phantom (the dynamic range is between 0.0 and 2.0)
let phantom = shepplogan(nx, ny);

// Modified Shepp-Logan Phantom (the dynamic range is between 0.0 and 1.0)
let phantom_modified = shepplogan_modified(nx, ny);
```

See `examples/example.rs` for an example which saves the phantom to disk.

You can also create your own phantom by defining ellipses:

```rust
extern crate shepplogan;
use shepplogan::{phantom, Ellipse};

// Dimensions of the image grid
let (nx, ny) = (256, 320);

// Define two ellipses
let ellipses = 
    [
        Ellipse::new(0.0, -0.0184, 0.6624, 0.874, 0.0, -0.98),
        Ellipse::new(0.0, 0.0, 0.69, 0.92, 0.0, 2.0),
    ];

let ph = phantom(&ellipses, nx, ny);
```

This will create a phantom consisting of two ellipses.

## References

[0] Shepp, LA and Logan BF, "The Fourier reconstruction of a head section." IEEE Transactions
on Nuclear Science 21, No. 3 (1974)

[1] Toft, PA, "The Radon Transform - Theory and Implementation", PhD dissertation, Departement
of Mathematical Modelling, Technical University of Denmark (1996)


## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](https://github.com/stefan-k/shepplogan-rs/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT License ([LICENSE-MIT](https://github.com/stefan-k/shepplogan-rs/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
