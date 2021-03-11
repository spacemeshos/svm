#![no_std]
#![feature(maybe_uninit_uninit_array)]
#![feature(once_cell)]

//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

pub mod traits;

mod ext;
mod mock;

#[cfg(not(any(feature = "ffi", feature = "mock")))]
compile_error!("must have at least one feature flag turned-on (`ffi` or `mock`)");

#[cfg(feature = "ffi")]
pub use ext::ExtHost;

#[cfg(feature = "mock")]
pub use mock::MockHost;
