#![allow(missing_docs)]
#![allow(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

pub mod default;
pub mod memory;
pub mod traits;
pub mod types;
pub mod wasm;

mod wire;

pub use wire::build_wasm_contract;
pub use wire::ContractError;
