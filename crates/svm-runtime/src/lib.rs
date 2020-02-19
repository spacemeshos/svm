#![allow(missing_docs)]
#![allow(unused)]

//! `SVM-runtime` crate is the glue between `SVM` to a Wasm Runtime

/// crate traits goes here
pub mod traits;

/// Implements the most high-level API of `SVM`.
mod runtime;

pub use runtime::{create_rocksdb_runtime, DefaultRuntime, Receipt};

/// Implements `SvmCtx`. Used for running `SVM` instances.
pub mod ctx;

pub mod register;

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

/// Wasm integer values (I32 / I64)
pub mod value;
