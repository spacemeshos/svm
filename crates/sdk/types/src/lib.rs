//! This crate implements SDK types or SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (not using Rust standard library) annotation in order to reduce the compiled WASM size.

#![no_std]
#![feature(maybe_uninit_uninit_array)]
#![feature(core_intrinsics)]
#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

/// SDK types
pub mod types;

/// SDK values
pub mod value;

mod amount;
pub use amount::Amount;

mod layer_id;
pub use layer_id::LayerId;

mod blob;
pub use blob::Address;
