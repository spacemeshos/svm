#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! `SVM-runtime` crate is the glue between `SVM` to a Wasm Runtime
//!
//! Currently there is one a single `Runtime` implementation supporting `Wasmer`,
//! But future WASM Runtime might be added.

/// Runtime Receipts
pub mod receipt;

/// Implements the most high-level API of `SVM`.
mod runtime;
pub use runtime::{create_rocksdb_runtime, Config, DefaultRuntime, Runtime};

/// Gas estimation and metering.
pub mod gas;

mod storage;

/// Implements `SvmCtx`. Used for running `SVM` instances.
pub mod ctx;

/// `Buffer` used for for running `App`s.
pub mod buffer;

/// Implements the helpers to be consumed by `SVM` vmcalls.
#[macro_use]
pub mod helpers;

/// Implements common functionalities to be consnumed by tests.
pub mod testing;

/// Implements the `SVM` vmcalls (a.k.a libcalls / hostcalls / syscalls)
pub mod vmcalls;

/// Crates errors
pub mod error;
