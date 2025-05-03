#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! The `s2json` Rust crate provides functionalities to read and write S2JSON Spec data structures.
//! This crate is a 0 dependency package that uses `no_std` and is intended to be used in
//! embedded systems and WASM applications.
//! NOTE: WG stands for WGS84 and S2 stands for S2Geometry

extern crate s2json_core;
#[cfg(feature = "derive")]
extern crate s2json_derive;

pub use s2json_core::*;
#[cfg(feature = "derive")]
pub use s2json_derive::*;
