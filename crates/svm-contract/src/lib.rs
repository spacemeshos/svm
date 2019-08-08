#![deny(missing_docs)]
#![deny(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

mod code_hash;
mod traits;
mod types;
mod wasm;
mod wire;

pub use wire::parse::parse_contract;
