[![Dependency status](https://deps.rs/repo/github/myst6re/lzs/status.svg)](https://deps.rs/repo/github/myst6re/lzs)
[![crates.io](https://img.shields.io/crates/v/lzs.svg)](https://crates.io/crates/lzs)

# crate lzs

<!-- cargo-rdme start -->

## Lempel–Ziv–Storer–Szymanski de-/compression

`LZSS` is a lossless data compression algorithm in pure Rust.
This crate is built for embedded systems:

* Small code size
* Uses little RAM and CPU
* `no_std` feature

## lzss crate VS lzs crate

This crate (lzs) implements an early version of the LZSS algorithm published by
Haruhiko Okumura in 1989.

In this version, only the initial character (C) in configurable.

The lzss crate implements a version of LZSS that can work bit by bit, instead of byte by byte.
Also the structure is different, meaning lzss crate output is incompatible with lzs crate output.

## Lack of a header

This algorithm has by design no header at all. Please be aware that it is not
possible to check if the contents is correct, or even the length matches.
It is recommended to add a header based on the requirements.

## Origin
This code is based on the [LZSS encoder-decoder by Haruhiko Okumura, public domain](http://oak.oakland.edu:80/pub/simtelnet/msdos/arcutils/lz_comp2.zip).

In order to create an encoder-decoder which is compatible to the program above
the following is required: `C = 0x20`

## Features
* `alloc`       - Allows de-/compression with buffer on the heap and the `VecWriter`.
* `safe`        - Only use safe code (see Safety below).
* `std`         - Enables `alloc` and additional `IOSimpleReader`, `IOSimpleWriter`,
                  and the `Error` instance for `LzsError`.

`std` and `safe` are enabled by default.

### Usage
With defaults (`std` and `safe`):
```toml
[dependencies]
lzs = "0.9"
```

With `no_std` (and without `safe`):
```toml
[dependencies]
lzs = { version = "0.9", default-features = false }
```

## Example
```rust
let input = b"Example Data";
let mut output = [0; 30];
let result = Lzs::new(0x20).compress(
  SliceReader::new(input),
  SliceWriter::new(&mut output),
);
assert_eq!(result, Ok(14)); // there was no overflow and the output is 14 bytes long
```

## Safety

With the `safe` feature the code is not using any unsafe code (`forbid(unsafe_code)`), but at
the cost of performance and size - though on modern systems that is not to mention.

But on smaller systems (like microcontrollers, where `no_std` is needed) it may be noticeable.
Which is the reason wht it can be switched on/off.

<!-- cargo-rdme end -->

# Command-Line-Interface

In oder to de-/compress files in the cli, install lzs-cli:

```shell
cargo install lzs-cli
```

Example:
```shell
lzs e 10,4,0x20 <input >outout
```
