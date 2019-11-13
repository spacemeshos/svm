#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible on doing gas estimations for SVM contracts.
//! SVM contracts are essentially wasm programs importing the SVM vmcalls.

pub(crate) mod block;
pub(crate) mod cursor;
mod function;
pub(crate) mod program;

pub mod code_reader;

/// Gas estimation error
pub mod error;

/// Gas required for executing SVM contracts.
mod gas;

/// Implements the gas estimation logic
mod estimate;

pub use estimate::estimate_program;
pub use function::FuncIndex;
pub use gas::Gas;

pub mod traits;
