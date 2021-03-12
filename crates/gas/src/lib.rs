#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! This crate is responsible for doing gas estimation for apps.
//! SVM apps are essentially wasm programs importing SVM vmcalls.

mod block;
mod call_graph;
mod code_reader;
mod estimate;
mod function;
mod op;
mod program;
mod validation;

mod gas;
pub use gas::Gas;

/// Crate errors
pub mod error;

/// This is the place for the crate traits
pub mod traits;

pub use estimate::estimate_code;
pub use function::FuncIndex;
pub use validation::validate_code;
