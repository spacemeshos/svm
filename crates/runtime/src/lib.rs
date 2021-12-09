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

mod error;
mod func_env;
mod price_registry;
mod runtime;
mod wasm_store;

pub mod testing;
pub mod vmcalls;

pub use error::ValidateError;
pub use func_env::{FuncEnv, AccessMode};
pub use price_registry::PriceResolverRegistry;
pub use runtime::{compute_account_addr, compute_template_addr, Runtime};
pub use wasm_store::new_store;
