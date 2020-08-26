#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! `SVM-runtime` crate is the glue between `SVM` to a Wasm Runtime
//!
//! Currently there is one a single `Runtime` implementation supporting `Wasmer`,
//! But future WASM Runtime might be added.

/// Implements the most high-level API of `SVM`.
mod runtime;
pub use runtime::{create_rocksdb_runtime, Config, DefaultRuntime, Runtime};

/// Gas estimation and metering.
pub mod gas;

mod storage;

pub mod env;

/// Implements `Context`. Used for managing data of running `SVM` apps.
mod context;
pub use context::Context;

/// Implements the helpers to be consumed by `SVM` vmcalls.
#[macro_use]
pub mod helpers;

/// Implements common functionalities to be consnumed by tests.
pub mod testing;

/// Implements the `SVM` vmcalls (a.k.a libcalls / hostcalls / syscalls)
pub mod vmcalls;

/// Crate errors
pub mod error;
