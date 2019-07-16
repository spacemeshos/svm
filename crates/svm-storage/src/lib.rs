#![deny(missing_docs)]
#![allow(unused)]

//! `svm-storage` crate is responsible on the contract storage part of the `svm`
//! Each smart contract has its own storage

mod default_page_hasher;
mod default_pages_storage;
mod page;
mod page_cache_impl;
mod page_slice_cache;

pub use page::{PageIndex, SliceIndex};
pub use page_cache_impl::PageCacheImpl;
pub use page_slice_cache::{PageSliceCache, PageSliceLayout};

use default_page_hasher::DefaultPageHasher;
use default_pages_storage::DefaultPagesStorage;

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

        pub use mem_kv_store::MemKVStore;
        pub use mem_pages::MemPages;
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

cfg_if! {
    if #[cfg(feature = "svm_trie")] {
        mod trie_kv;
        mod trie_null_hasher;

        pub use trie_kv::TrieKV;
        pub use trie_null_hasher::TrieNullHasher;
    }
}
