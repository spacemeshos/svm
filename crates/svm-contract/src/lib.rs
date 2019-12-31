#![deny(missing_docs)]
#![deny(unused)]

//! The `svm-app` crate is responsible on `Spacemesh` `AppTemplate`(s) and `App`(s).

/// Default implementations for `AppTemplateAddressCompute` and `AppTemplateAddressCompute`
pub mod default;

/// AppTemplate environment
pub mod env;

/// In-memory environment and store
pub mod memory;

/// crate traits goes here
pub mod traits;

/// Transactions for executing a contract
pub mod transaction;

/// Common crate types
pub mod types;

/// Wasm contract and other related primitives
pub mod wasm;

/// `rocksdb` backed implementation for `AppTemplateStore` and `AppTemplateEnv`
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;

mod wire;

/// Exposed errors
pub mod error {
    pub use crate::wire::deploy::AppTemplateBuildError;
    pub use crate::wire::exec::TransactionBuildError;
}

/// Building in-memory representations for a new `AppTemplate / App` transaction
pub mod build {
    pub use crate::wire::deploy::WireAppTemplateBuilder;
    pub use crate::wire::exec::WireTxBuilder;
}
