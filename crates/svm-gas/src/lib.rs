#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible on doing gas estimations for SVM contracts.
//! SVM contracts are essentially wasm programs importing the SVM vmcalls.

pub(crate) mod block;
pub(crate) mod code_reader;
pub(crate) mod cursor;
pub(crate) mod function;
pub(crate) mod program;

/// Gas estimation error
pub mod error;

/// Gas required for executing SVM contracts.
pub mod gas;

/// Implements the gas estimation logic
pub mod function_gas;

pub mod estimate;
