#![allow(missing_docs)]
#![allow(unused)]

//! `svm-storage` crate is responsible on the contract storage part of the `svm`
//! Each smart contract has its own storage

/// Default implementations for crate traits (see `traits.rs`).
pub mod default;

mod contract_pages;

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
        /// in-memory backed implementation for storage
        pub mod memory;
    }
}

cfg_if! {
    if #[cfg(feature = "svm_rocksdb")]  {
        /// `rocksdb` backed implementation for storage
        pub mod rocksdb;
    }
}
