#![deny(missing_docs)]
#![deny(unused)]

//! This crate is responsible on providing a [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `wasmer svm`.

/// Will contain an `extern "C"` API to initiate a `wasmer svm` instance.
pub mod instance;
