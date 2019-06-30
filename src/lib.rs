#![deny(missing_docs)]
#![allow(unused)]

//! This `svm` crate is responsible on implementing the Spacemesh Virtual Machine for running Smart Contracts
//! under _Spacemesh_ Golang Full Node and the future Rust Full Node

extern crate wasmer_runtime;
extern crate wasmparser;

mod parser;

pub use parser::parse_wasm;
