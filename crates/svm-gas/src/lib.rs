#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible on doing gas estimations for SVM contracts.
//! SVM contracts are essentially wasm programs importing the SVM vmcalls.

pub(crate) mod block;
pub(crate) mod cursor;
pub(crate) mod program;

mod function;

/// Gas required for executing SVM contracts.
mod gas;

pub mod code_reader;
pub mod traits;

/// Gas estimation error
pub mod error;

/// Implements the gas estimation logic
mod estimate;

pub use estimate::estimate_program;
pub use function::FuncIndex;
pub use gas::Gas;
