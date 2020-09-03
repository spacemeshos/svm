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

/// SDK types
pub mod types;

/// SDK values
pub mod value;

/// Memory allocation on Heap
pub mod memory;

mod amount;
mod host;
mod layer_id;
mod log;
mod storage;

pub use log::log;

pub use amount::Amount;
pub use host::Host;
pub use layer_id::LayerId;
pub use storage::Storage;

/// `ensure` macro
#[macro_use]
pub mod ensure;
