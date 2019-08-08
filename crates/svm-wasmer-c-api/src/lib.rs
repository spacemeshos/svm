#![deny(missing_docs)]
#![allow(unused)]

//! This crate is responsible on providing a [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `wasmer svm`.

/// Contains macros for the `svm wasmer C-API`
pub mod macros;

/// An in-memory implemention of `C-API` of `svm wasmer`
pub mod mem_c_api;

/// Types to be used for FFI integration.
pub mod c_types;
