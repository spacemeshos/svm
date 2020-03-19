#![deny(missing_docs)]
#![deny(unused)]

//! `SVM-runtime` crate is the glue between `SVM` to a Wasm Runtime

/// Runtime Receipts
pub mod receipt;

/// Implements the most high-level API of `SVM`.
pub mod runtime;
pub use runtime::{create_rocksdb_runtime, DefaultRuntime, Runtime};

/// Gas estimation and metering.
pub mod gas;

mod storage;

/// Implements `SvmCtx`. Used for running `SVM` instances.
pub mod ctx;

/// `Register` used for for running `App`s.
pub mod register;

/// `Buffer` used for for running `App`s.
pub mod buffer;

/// Implements the helpers to be consumed by `SVM` vmcalls.
#[macro_use]
pub mod helpers;

/// Implements common functionalities to be consnumed by tests.
pub mod testing;

/// Implements the `SVM` vmcalls (a.k.a libcalls / hostcalls / syscalls)
pub mod vmcalls;

/// Options when spawning a new `SVM` runtime instance
pub mod settings;

/// Crates errors
pub mod error;
