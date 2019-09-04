#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible on providing a [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `wasmer svm`.

/// C-API for the `svm runtime`
pub mod c_api;

/// An in-memory implemention of `svm runtime` C-API
pub mod mem_c_api;

/// Types to be used for FFI integration.
pub mod c_types;

/// C-API utilities to be used primarily for tests / integration-tests
pub mod c_utils;
