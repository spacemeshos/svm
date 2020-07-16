#![no_std]
#![feature(core_intrinsics)]

//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![deny(missing_docs)]
#![allow(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

/// SDK types
pub mod types;

/// SDK values
pub mod value;

mod log;

/// Logging API
pub use log::log;

/// `ensure` macro
#[macro_use]
pub mod ensure;
