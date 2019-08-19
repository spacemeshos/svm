#![allow(missing_docs)]
#![allow(unused)]

//! `svm-storage` crate is responsible on the contract storage part of the `svm`
//! Each smart contract has its own storage

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
        pub mod memory;
    }
}

cfg_if! {
    if #[cfg(feature = "svm_leveldb")]  {
        // mod level_key;
        // mod leveldb_kv;
        //
        // pub use level_key::*;
        // pub use leveldb_kv::LevelDB;
    }
}
