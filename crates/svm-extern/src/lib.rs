#![deny(missing_docs)]
#![deny(unused)]

//! `svm-extern` crate contains the `extern "C"` interface that will be consumed by `SVM` apps
//! written in Rust

/// Interfaces for interacting with the Node.
pub mod node;

/// Interfaces for interacting with the app-storage.
pub mod storage;

/// Interfaces for arithmetic calculations on registers
pub mod register;
