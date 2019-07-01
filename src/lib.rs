#![deny(missing_docs)]
#![allow(unused)]

//! This `svm` crate is responsible on implementing the Spacemesh Virtual Machine for running Smart Contracts
//! under _Spacemesh_ Golang Full Node and the future Rust Full Node

extern crate hash256_std_hasher;
extern crate hash_db;
extern crate memory_db;
extern crate tiny_keccak;
extern crate trie_db;
extern crate wasmer_runtime;
extern crate wasmer_runtime_c_api;
extern crate wasmer_singlepass_backend;
extern crate wasmparser;

#[macro_use]
mod compiler;

mod middleware;
mod package;
mod vm;
