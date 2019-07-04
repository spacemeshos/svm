#![deny(missing_docs)]
#![deny(unused)]

//! under _Spacemesh_ Golang Full Node and the future Rust Full Node
//! This `svm` crate is responsible on implementing the Spacemesh Virtual Machine for running Smart Contracts

#[macro_use]
mod compiler;

mod middleware;
mod vm;

pub use compiler::compile_program;
