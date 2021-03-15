#![feature(maybe_uninit_uninit_array)]

//! This crate implements SDK types or SVM.
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

mod amount;
pub use amount::Amount;

mod layer_id;
pub use layer_id::LayerId;

mod blob;
pub use blob::Address;
