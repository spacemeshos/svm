#![deny(missing_docs)]
#![deny(unused)]

//! `svm-extern` crate contains the `extern "C"` interface that will be consumed by `SVM` apps
//! written in Rust

/// Interface for interacting with the Node.
pub mod node;

/// Interface for interacting with the app-storage.
pub mod storage;

/// Interface for interacting with registers.
pub mod register;

/// Interface for interacting with buffers.
pub mod buffer;

/// Interface for interacting with the `HostCtx`.
pub mod host_ctx;
