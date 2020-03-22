#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! The `svm-kv` is responsible on providing different implementations for the `KVStore` trait.
//! (defined in `traits.rs`).

/// Defines the `KVStore` trait.
pub mod traits;

/// An in-memory implementation for `KVStore`
#[cfg(feature = "memory")]
pub mod memory;

/// `KVStore` backed by rocksdb
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;
