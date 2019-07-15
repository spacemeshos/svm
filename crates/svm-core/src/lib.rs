#![deny(missing_docs)]
#![deny(unused)]

//! This `svm` crate is responsible on implementing the Spacemesh Virtual Machine for running Smart Contracts
//! under _Spacemesh_ Golang Full Node and the future Rust Full Node

#[macro_use]
mod compiler;

mod middleware;

pub use compiler::compile_program;
