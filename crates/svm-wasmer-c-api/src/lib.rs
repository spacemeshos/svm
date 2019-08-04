#![deny(missing_docs)]
// #![deny(unused)]
#![allow(unused)]

//! This crate is responsible on providing a [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `wasmer svm`.

/// Will contain macros for the `svm wasmer C-API`
pub mod macros;

/// Will contain an in-memory implemention of `C-API` of `svm wasmer`
pub mod mem_c_api;
