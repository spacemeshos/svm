#![allow(missing_docs)]
#![allow(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

pub mod default;
pub mod env;
pub mod memory;
pub mod traits;
pub mod types;

mod wasm;
mod wire;

pub use wire::build_wasm_contract;
pub use wire::ContractError;
pub use wire::WireContractBuilder;
