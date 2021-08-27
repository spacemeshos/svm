//! This crate is the glue between `SVM` to a Wasm Runtime.
//!
//! Currently there is one a single [`Runtime`] implementation supporting
//! [`Wasmer`](https://wasmer.io/), but future WASM Runtimes might be added.

#![deny(missing_docs)]
#![deny(unused)]
#![warn(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(vec_into_raw_parts)]

mod env;
mod error;
mod func_env;
mod runtime;
mod storage;
mod wasm_store;

pub mod testing;
pub mod vmcalls;

pub use env::{Env, EnvTypes};
pub use error::ValidateError;
pub use func_env::{FuncEnv, ProtectedMode};
pub use runtime::{Config, DefaultRuntime, Runtime};
pub use wasm_store::new_store;

#[cfg(feature = "default-rocksdb")]
pub use runtime::create_rocksdb_runtime;
