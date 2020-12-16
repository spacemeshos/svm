#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

//! `SVM-runtime` crate is the glue between `SVM` to a Wasm Runtime
//!
//! Currently there is one a single `Runtime` implementation supporting `Wasmer`,
//! But future WASM Runtime might be added.

/// Implements the most high-level API of `SVM`.
mod runtime;
pub use runtime::{Config, DefaultRuntime, Runtime, RuntimePtr};

#[cfg(feature = "default-rocksdb")]
pub use runtime::create_rocksdb_runtime;

/// Gas estimation and metering.
pub mod gas;

mod import;
mod storage;

pub mod env;
pub use import::ExternImport;

/// Implements `Context`. Used for managing data of running `SVM` apps.
mod context;
pub use context::Context;

/// Implements common functionalities to be consnumed by tests.
pub mod testing;

/// Implements the `SVM` vmcalls (a.k.a libcalls / hostcalls / syscalls)
pub mod vmcalls;

/// Crate errors
pub mod error;
