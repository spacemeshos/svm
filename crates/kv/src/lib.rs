#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! The `svm-kv` crate is responsible on providing different implementations for the `KVStore` trait.

/// Defines the `RawKV` trait.
pub mod traits;

/// Helpers for composing keys.
pub mod key;

#[cfg(not(any(feature = "default-memory", feature = "default-rocksdb")))]
compile_error!("should be compiled with feature `default-memory` or `default-rocksdb`");

/// An in-memory implementation for `KVStore`
#[cfg(feature = "default-memory")]
pub mod memory;

/// `KVStore` backed by rocksdb
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;
