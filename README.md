# portable-io

A subset of Rust `std::io` functionality supported for `no-std`.

## requirements

- XXX Rust nightly toolchain required for XXX features - XXX nightly MSRV: `nightly-2022-08-24`
- XXX enable `--cfg portable_io_unstable_all` Rust flag - XXX IN CASE OF XXX FEATURES - XXX TBD SHOULD THIS BE FINER-GRAINED ???

XXX TODO MSRV FOR RUST STABLE

## major TODO items

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
