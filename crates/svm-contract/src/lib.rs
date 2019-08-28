#![allow(missing_docs)]
#![allow(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

pub mod default;
pub mod env;
pub mod memory;
pub mod traits;
pub mod types;
pub mod wasm;

mod transaction;
mod wire;

pub use transaction::Transaction;
pub use wire::deploy::ContractDeployError;
pub use wire::deploy::WireContractBuilder;
pub use wire::exec::parse_transaction;
pub use wire::exec::ContractExecError;
pub use wire::exec::WireTxBuilder;
