#![deny(missing_docs)]
#![deny(unused)]

//! `svm-extern` crate contains the `extern "C"` interfaces that will be consumed by `SVM` Rust Smart-Contracts

/// Interfaces for interacting with the Node.
pub mod node;

/// Interfaces for interacting with the Contract Storage.
pub mod storage;

/// Interfaces for arithmetic calculations on registers
pub mod register;
