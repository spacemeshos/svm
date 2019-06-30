#![deny(missing_docs)]
#![allow(unused)]

//! This `svm` crate is responsible on implementing the Spacemesh Virtual Machine for running Smart Contracts
//! under _Spacemesh_ Golang Full Node and the future Rust Full Node

extern crate wasmer_runtime;
extern crate wasmer_runtime_c_api;
extern crate wasmer_singlepass_backend;
extern crate wasmparser;

mod middleware;
mod vm;
