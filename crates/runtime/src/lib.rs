//! `SVM-runtime` crate is the glue between `SVM` to a Wasm Runtime.
//!
//! Currently there is one a single [`Runtime`] implementation supporting
//! [`Wasmer`](https://wasmer.io/), but future WASM Runtime might be added.

#![warn(missing_docs)]
#![deny(unused)]
#![warn(dead_code)]
#![deny(unreachable_code)]
#![feature(vec_into_raw_parts)]

mod context;
mod env;
mod error;
mod runtime;
mod storage;
pub mod testing;
pub mod vmcalls;
mod wasm_store;

pub use context::Context;
pub use env::{Env, EnvTypes};
pub use error::ValidateError;
#[cfg(feature = "default-rocksdb")]
pub use runtime::create_rocksdb_runtime;
pub use runtime::{Config, DefaultRuntime, Runtime, RuntimePtr};
pub use wasm_store::new_store;
