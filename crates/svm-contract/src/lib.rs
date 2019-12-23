#![deny(missing_docs)]
#![deny(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

/// Default implementations for `ContractAddressCompute` and `ContractAddressCompute`
pub mod default;

/// Contract environment
pub mod env;

/// In-memory Contract environment and store
pub mod memory;

/// crate traits goes here
pub mod traits;

/// Transactions for executing a contract
pub mod transaction;

/// Common crate types
pub mod types;

/// Wasm contract and other related primitives
pub mod wasm;

/// `rocksdb` backed implementation for `ContractStore` and `ContractEnv`
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;

mod wire;

/// Exposed errors
pub mod error {
    pub use crate::wire::deploy::ContractBuildError;
    pub use crate::wire::exec::TransactionBuildError;
}

/// Building in-memory representations for a new contract / smart-contract transaction
pub mod build {
    pub use crate::wire::deploy::WireContractBuilder;
    pub use crate::wire::exec::WireTxBuilder;
}
