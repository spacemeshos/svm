#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! The `svm-app` crate is responsible on `Spacemesh` `AppTemplate`(s) and `App`(s).

/// Default implementations for `AppTemplateAddressCompute` and `AppTemplateAddressCompute`
pub mod default;

/// In-memory environment and store
pub mod memory;

/// Contains crate traits
pub mod traits;

/// Common crate types such as `App`, `AppTemplate` and other related primitives
pub mod types;

/// `rocksdb` backed implementation for `AppTemplateStore` and `AppTemplateEnv`
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;

/// Parsing raw representations of a `AppTemplate / App / AppTransaction`
pub mod raw;

/// Crate errors
pub mod error;

/// Testing helpers
pub mod testing;
