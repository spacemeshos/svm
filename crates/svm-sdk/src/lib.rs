#![no_std]

//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

pub mod storage;

mod amount;
pub use amount::Amount;

/// SDK types
pub mod types;

/// SDK values
pub mod value;

mod blob;
pub use blob::Address;

/// Memory allocation on Heap
pub mod memory;

mod log;

/// Logging API
pub use log::log;

/// `ensure` macro
#[macro_use]
pub mod ensure;
