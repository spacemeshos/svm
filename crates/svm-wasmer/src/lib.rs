// #![deny(missing_docs)]
// #![deny(unused)]
#![allow(unused)]

//! `svm-wasmer` crate is the glue between `svm` constract storage to `wasmer` live instances
mod wasmer_register;

/// `wasmer_storage` implements the high-level API to be consumed by `wasmer` instances
pub mod wasmer_storage;
