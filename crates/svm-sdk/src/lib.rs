#![no_std]
#![feature(maybe_uninit_uninit_array)]

//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

mod log;

/// Logging API
pub use log::log;

/// `ensure` macro
#[macro_use]
pub mod ensure;

use svm_sdk_alloc;
use svm_sdk_host;
use svm_sdk_storage;
use svm_sdk_types;

pub use svm_sdk_alloc::*;
pub use svm_sdk_host::*;
pub use svm_sdk_storage::*;
pub use svm_sdk_types::*;
