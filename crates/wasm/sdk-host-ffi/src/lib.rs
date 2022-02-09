//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust standard-library) annotation in order to reduce the compiled WASM size.

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(maybe_uninit_uninit_array)]
#![feature(once_cell)]

mod ext;

pub use ext::ExtHost;
