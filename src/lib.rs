#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types)]

#[cfg(feature = "buildtime-bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "buildtime-bindgen"))]
mod bindings;

#[cfg(not(feature = "buildtime-bindgen"))]
pub use bindings::*;
