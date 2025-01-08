# portable-io

A subset of Rust `std::io` functionality supported for `no-std`.

## requirements

- enable `alloc` feature
- Rust stable MSRV: `1.81.0` / nightly MSRV: `nightly-2022-08-24`
- some of the functionality requires Rust nightly together with unstable configuration `--cfg portable_io_unstable_all` in Rust flags (set `RUSTFLAGS` env variable when running `cargo build` or `cargo test`)
- unstable configuration `--cfg portable_io_unstable_all` in Rust flags is required for Rust nightly pre-`2024-06-09` to enable `error_in_core` feature directive (stabilized in June 2024)

## major TODO items

- ~~Resolve build warnings~~
- Resolve doc warnings
- Finer-grained feature options
- Include updates from newer versions of upstream Rust library code
- Include & adapt some more documentation from upstream Rust library code
- Resolve other TODO items in this code

## license

[MIT](./LICENSE-MIT) or [Apache 2.0](./LICENSE-APACHE) license options

NOTE that this code is adapted from Rust `std` library code version `1.59.0`
(last release version that did not require unstable `macro` feature to build `error` module)
