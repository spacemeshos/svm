#![allow(missing_docs)]
#![allow(unused)]

//! `svm-storage` crate is responsible on the contract storage part of the `svm`
//! Each smart contract has its own storage

/// Default implementations for crate traits (see `traits.rs`).
pub mod default;

mod merkle_pages_storage;

mod page_slice_cache;

/// Contains definitions of `Page` related structures. For example: `Page` / `PageIndex` / `SliceIndex`
pub mod page;

/// Contains definitions `State`-related.
pub mod state;

pub use crate::page_slice_cache::PageSliceCache;

/// Storage related traits
#[macro_use]
pub mod traits;

/// Common storage macros
#[macro_use]
pub mod macros;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "svm_memory")] {
        /// Contains;
        /// * `MemKVStore`         - An in-memory implementation for `KVStore`
        /// * `MemMerklePages`     - An in-memory implementation for `PagesStorage`
        /// * `MemMerklePageCache` - An in-memory implementation for `PageCache`
        pub mod memory;
    }
}

cfg_if! {
    if #[cfg(feature = "svm_leveldb")]  {
        /// `LDBStore` - An implementation of `KVStore` against `LevelDB`
        pub mod leveldb;
    }
}
