#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! The `svm-kv` crate is responsible on providing different implementations for the `KVStore` trait.

/// Defines the `RawKV` trait.
pub mod traits;

/// Helpers for composing keys.
pub mod key;

/// An in-memory implementation for `KVStore`
#[cfg(feature = "memory")]
pub mod memory;

/// `KVStore` backed by rocksdb
#[cfg(feature = "default-rocksdb")]
pub mod rocksdb;
