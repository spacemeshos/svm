#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! This crate is responsible on managing the App's storage.
//!
//! That includes the specification of the app's data-layout,
//! and execution of operations against the app's storage (reads and writes).

mod raw;

/// High-level `AppStorage`
pub mod app;

/// Key-value abstraction
pub mod kv;

/// App's data-layout
pub mod layout;
