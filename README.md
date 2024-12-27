# portable-io

A subset of Rust `std::io` functionality supported for `no-std`.

## requirements

- Rust nightly toolchain - MSRV: `nightly-2022-08-24`
- enable `--cfg portable_io_unstable_all` Rust flag
- enable `alloc` feature

## major TODO items

- Support building with stable Rust version, likely with only a subset of features possible
- Resolve build warnings
- Resolve doc warnings
- Finer-grained feature options
- Include updates from newer versions of upstream Rust library code
- Include & adapt some more documentation from upstream Rust library code
- Resolve other TODO items in this code

## license

[MIT](./LICENSE-MIT) or [Apache 2.0](./LICENSE-APACHE) license options

NOTE that this code is adapted from Rust `std` library code version `1.59.0`
(last release version that did not require unstable `macro` feature to build `error` module)
