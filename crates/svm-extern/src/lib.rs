#![deny(missing_docs)]
#![deny(unused)]

//! `svm-extern` crate contains the `extern "C"` interfaces that will be consumed by Rust Smart Contracts

mod storage;

pub use crate::storage::*;
