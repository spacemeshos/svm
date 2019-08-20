#![allow(missing_docs)]
#![allow(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

pub mod traits;
pub mod types;
pub mod wasm;

mod code_hash;
mod default;
mod wire;

pub use wire::build_wasm_contract;
