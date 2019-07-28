#![deny(missing_docs)]
// #![deny(unused)]
#![allow(unused)]

//! This crate is responsible on providing a [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `wasmer svm`.

/// Will contain an `extern "C"` API for `svm wasmer`
pub mod c_api;
