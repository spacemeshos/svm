#![allow(missing_docs)]
#![deny(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

/// Default implementations for `ContractAddressCompute` and `ContractAddressCompute`
pub mod default;

pub mod env;
pub mod memory;
pub mod traits;
pub mod transaction;
pub mod types;
pub mod wasm;

/// `rocksdb` backed implementation for `ContractStore` and `ContractEnv`
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;

mod wire;

pub mod error {
    pub use crate::wire::deploy::ContractBuildError;
    pub use crate::wire::exec::TransactionBuildError;
}

/// Building in-memory representations for a new contract / smart-contract transaction
pub mod build {
    pub use crate::wire::deploy::WireContractBuilder;
    pub use crate::wire::exec::WireTxBuilder;
}
