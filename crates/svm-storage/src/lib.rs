#![deny(missing_docs)]
#![allow(unused)]

//! `svm-storage` crate is responsible on the contract storage part of the `svm`
//! Each smart contract has its own storage

mod default_page_cache;
mod default_page_index_hasher;
mod default_pages_storage;
mod merkle_page_storage;
mod page_slice_cache;

/// Contains definitions of common page related structures. For example: `Page` / `PageIndex` / `SliceIndex`
pub mod page;

use merkle_page_storage::MerklePageStorage;

pub use crate::page_slice_cache::PageSliceCache;

/// Contains `svm storage` related default implementations for traits defined under the `traits` module.
pub mod default {
    pub use crate::default_page_cache::DefaultPageCache;
    pub use crate::default_page_index_hasher::DefaultPageIndexHasher;
    pub use crate::default_pages_storage::DefaultPagesStorage;
}

/// Do-nothing implementation for various storage related abstractions.
/// Very usable for code requiring a storage dependencies it doesn't care about
pub mod null_storage;

/// Storage related traits
#[macro_use]
pub mod traits;

/// Common storage macros
#[macro_use]
pub mod macros;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "svm_memory")] {
        mod mem_kv_store;
        mod mem_pages;

        /// Implements `svm storage` related in-memory data-structures.
        pub mod memory {
            pub use crate::mem_kv_store::MemKVStore;
            pub use crate::mem_pages::MemPages;
        }
    }
}

cfg_if! {
    if #[cfg(feature = "svm_leveldb")]  {
        mod level_key;
        mod leveldb_kv;

        pub use level_key::*;
        pub use leveldb_kv::LevelDB;
    }
}

// cfg_if! {
//     if #[cfg(feature = "svm_trie")] {
//     }
// }
