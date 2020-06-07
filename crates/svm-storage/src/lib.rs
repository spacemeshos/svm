#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![feature(const_mut_refs)]

//! This crate is responsible on managing the App's storage.
//!
//! That includes the specification of the app's data-layout,
//! and execution of operations against the app's storage (reads and writes).

/// High-level `AppStorage`
pub mod app;

/// Key-value abstraction
pub mod kv;

/// Tests helpers
pub mod testing;
