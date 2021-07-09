//! `SVM-runtime` crate is the glue between `SVM` to a Wasm Runtime.
//!
//! Currently there is one a single [`Runtime`] implementation supporting
//! [`Wasmer`](https://wasmer.io/), but future WASM Runtime might be added.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

mod context;
mod env;
mod gas;
mod import;
mod runtime;
mod storage;

pub mod error;
pub mod testing;
pub mod vmcalls;

pub use context::Context;
pub use env::{Env, EnvTypes};
pub use gas::DefaultGasEstimator;
pub use import::ExternImport;
#[cfg(feature = "default-rocksdb")]
pub use runtime::create_rocksdb_runtime;
pub use runtime::{Config, DefaultRuntime, Runtime, RuntimePtr};
