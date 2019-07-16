#![deny(missing_docs)]
#![allow(unused)]

//! `svm-storage` crate is responsible on the contract storage part of the `svm`
//! Each smart contract has its own storage

mod default_page_hasher;
mod default_pages_storage;
mod mem_kv_store;
mod mem_pages;
mod page;
mod page_cache_impl;
mod page_slice_cache;

/// Common storage macros
#[macro_use]
pub mod macros;

/// Storage related traits
#[macro_use]
pub mod traits;

/// Do-nothing implementation for various storage related abstractions.
/// Very usable for code requiring a storage dependencies it doesn't care about
pub mod null_storage;

use default_page_hasher::DefaultPageHasher;
use default_pages_storage::DefaultPagesStorage;

#[cfg(feature = "memory_kv")]
pub use mem_kv_store::MemKVStore;

#[cfg(feature = "memory_pages")]
pub use mem_pages::MemPages;

pub use page::{PageIndex, SliceIndex};

pub use page_cache_impl::PageCacheImpl;
pub use page_slice_cache::{PageSliceCache, PageSliceLayout};
