#![deny(missing_docs)]
#![deny(unused)]

//! This crate is responsible on doing gas estimations for SVM contracts.
//! SVM contracts are essentially wasm programs importing the SVM vmcalls.

pub(crate) mod block;
pub(crate) mod program;

mod function;

/// Gas required for executing SVM contracts.
mod gas;

/// Reading wasm code wasm code
pub mod code_reader;

/// This is the place for the crate traits
pub mod traits;

/// Gas estimation error
pub mod error;

/// Implements the gas estimation logic
mod estimate;

pub use estimate::estimate_program;
pub use function::FuncIndex;
pub use gas::Gas;
