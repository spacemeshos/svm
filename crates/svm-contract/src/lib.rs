#![allow(missing_docs)]
#![allow(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

pub mod default;
pub mod env;
pub mod memory;
pub mod traits;
pub mod transaction;
pub mod types;
pub mod wasm;
mod wire;

pub mod error {
    pub use crate::wire::deploy::ContractBuildError;
    pub use crate::wire::exec::TransactionBuildError;
}

pub mod build {
    pub use crate::wire::deploy::WireContractBuilder;
    pub use crate::wire::exec::WireTxBuilder;
}
