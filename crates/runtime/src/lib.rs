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
mod gas;
pub use gas::DefaultGasEstimator;

mod context;
mod env;
mod storage;

pub use context::Context;
pub use env::{Env, EnvTypes};

pub mod error;
pub mod testing;
pub mod vmcalls;
