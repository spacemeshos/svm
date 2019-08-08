#![deny(missing_docs)]
#![deny(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

mod traits;
mod types;

mod wire;

pub use wire::parse::parse_contract;

mod default_code_hasher;
mod mem_code_hash_store;
mod wasm_contract;
