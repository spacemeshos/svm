#![allow(missing_docs)]
#![allow(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

pub mod traits;
pub mod types;
pub mod wasm;

mod code_hash;
mod null_contract_types;
mod wire;

pub use null_contract_types::NullContractTypes;
pub use wire::build_wasm_contract;
