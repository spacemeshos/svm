#![deny(missing_docs)]
#![deny(unused)]

//! The `svm-kv` is responsible on providing different implementations for the `KVStore` trait.
//! (defined in `traits.rs`).

/// Defines the `KVStore` trait.
pub mod traits;

/// An in-memory implementation for `KVStore`
#[cfg(feature = "memory")]
pub mod memory;

/// An `leveldb` backed implementation for `KVStore`
#[cfg(feature = "default-leveldb")]
pub mod leveldb;

/// An `rocksdb` backed implementation for `KVStore`
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;
