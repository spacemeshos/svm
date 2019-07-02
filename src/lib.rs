#![deny(missing_docs)]
#![allow(unused)]

//! This `svm` crate is responsible on implementing the Spacemesh Virtual Machine for running Smart Contracts
//! under _Spacemesh_ Golang Full Node and the future Rust Full Node

#[macro_use]
mod compiler;

mod common;
mod middleware;
mod package;
mod storage;
mod utils;
mod vm;

use common::Address;
pub use compiler::compile_program;
