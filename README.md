# portable-io

[![Crates.io Version](https://img.shields.io/crates/v/portable-io?style=flat-square)](https://crates.io/crates/portable-io)
[![Crates.io License](https://img.shields.io/crates/l/portable-io)](#license)

<!-- XXX TODO CHECK FOR DUPLICATED INFO HERE --->

<!-- XXX TODO CHECK FOR & RESOLVE TODO COMMENTS COPIED FROM lib.rs -->

<!-- cargo-sync-readme start -->

Traits, helpers, and type definitions for core I/O functionality.
A subset from Rust `std::io` functionality supported for `no-std`.

## Features

- `alloc` (enabled by default) - mandatory feature - for alloc-related functionality

## CFG options

- `portable_io_unstable_all` - enable all unstable options:
  - impl Write for Vec - uses Rust unstable `allocator_api` feature
  - size hint optimization for Read iterator - uses Rust unstable `min_specialization` feature

To enable: use `--cfg portable_io_unstable_all` in Rust flags, set `RUSTFLAGS` env variable
when running `cargo build` or `cargo test` for example.

<!-- TODO: MAINTAIN & VERIFY SYNC WITH README, POSSIBLY USING CARGO TOOL: cargo-sync-readme -->

<!-- TODO INCLUDE & ADAPT MORE DOC COMMENTS FROM RUST STD IO LIBRARY CODE -->

<!-- TODO: CLEANUP AS MANY CARGO DOC WARNINGS AS POSSIBLE & CHECK THIS IN CI -->

<!-- cargo-sync-readme end -->

## requirements

- enable `alloc` feature if using this package with no default features
- Rust stable MSRV: `1.81.0` / nightly MSRV: `nightly-2022-08-24`
- some of this functionality requires Rust nightly together with unstable configuration `--cfg portable_io_unstable_all` in Rust flags (set `RUSTFLAGS` env variable when running `cargo build` or `cargo test`):
  - impl Write for Vec - uses Rust unstable `allocator_api` feature
  - size hint optimization for Read iterator - uses Rust unstable `min_specialization` feature
- unstable configuration `--cfg portable_io_unstable_all` in Rust flags is required for Rust nightly pre-`2024-06-09` to enable `error_in_core` feature directive (stabilized in June 2024)

## major TODO items

- ~~Resolve build warnings~~
- Resolve doc warnings
- ~~Finer-grained feature options~~ _(for future consideration)_
- Include updates from newer versions of upstream Rust library code
- Include & adapt some more documentation from upstream Rust library code
- Resolve other TODO items in this code

## license

[MIT](./LICENSE-MIT) or [Apache 2.0](./LICENSE-APACHE) license options

NOTE that this code is adapted from Rust `std` library code version `1.59.0`
(last release version that did not require unstable `macro` feature to build `error` module)
