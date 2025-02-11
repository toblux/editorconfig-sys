# Rust FFI Bindings to libeditorconfig

This crate uses [bindgen](https://crates.io/crates/bindgen) and [pkg-config](https://crates.io/crates/pkg-config) to automatically generate Rust FFI bindings to the [editorconfig-core](https://github.com/editorconfig/editorconfig-core-c) C library.

Following the `*-sys` package convention, `editorconfig-sys` is just a thin wrapper around the native `libeditorconfig` library.

The safe Rust bindings to `libeditorconfig`, built on top of this `editorconfig-sys` crate, can be found [here](https://github.com/toblux/editorconfig-rs).

[![Build status](https://github.com/toblux/editorconfig-sys/actions/workflows/test.yml/badge.svg)](https://github.com/toblux/editorconfig-sys/actions)
[![Crates.io](https://img.shields.io/crates/v/editorconfig-sys.svg)](https://crates.io/crates/editorconfig-sys)

## Dependencies

To use this crate, `pkg-config` and `libeditorconfig >= 0.12.5` must be installed. For example, on Debian or Ubuntu you could use `apt install pkg-config libeditorconfig-dev` and on macOS `brew install pkg-config editorconfig` to install the dependencies.

You can check if `pkg-config` can find the library and which version is installed with:

```sh
pkg-config --modversion editorconfig
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
editorconfig-sys = "0.1.3"
```

## Usage

Some `unsafe` Rust code examples can be found in the [tests](tests/editorconfig_sys.rs).
