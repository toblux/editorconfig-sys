# Rust FFI Bindings to libeditorconfig

This crate uses [bindgen](https://crates.io/crates/bindgen) and [pkg-config](https://crates.io/crates/pkg-config) to automatically generate Rust FFI bindings to the [editorconfig-core](https://github.com/editorconfig/editorconfig-core-c) C library.

Following the `*-sys` package convention, `editorconfig-sys` is just a thin wrapper around the native `libeditorconfig` library.

<!-- The "safe" Rust bindings to `libeditorconfig`, built on top of this `editorconfig-sys` crate, can be found [here](https://github.com/toblux/rust-editorconfig). -->

![Workflow status](https://github.com/toblux/editorconfig-sys/actions/workflows/test.yml/badge.svg)

## Dependencies

To use this crate, `libeditorconfig >= 0.12.5` must be installed and `pkg-config` must be able to find it. You can check if `pkg-config` can find the library and which version is installed with:

```sh
pkg-config --modversion editorconfig
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
editorconfig-sys = "0.1.0"
```

## Usage

Some `unsafe` Rust code examples can be found in the [tests](tests/editorconfig_sys.rs).
