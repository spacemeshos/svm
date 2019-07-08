#![deny(missing_docs)]
#![allow(unused)]

//! `svm-storage` crate is responsible on the contract storage part of the `svm`
//! Each smart contract has its own storage

mod default_page_hasher;
mod mem_kv_store;
mod mem_pages;
mod page;
mod page_cache;
mod page_slice_cache;
mod pages_storage_impl;
mod traits;

use default_page_hasher::DefaultPageHasher;
use mem_kv_store::MemKVStore;
pub use mem_pages::MemPages;
use pages_storage_impl::PagesStorageImpl;

pub use page_cache::PageCache;
pub use page_slice_cache::PageSliceCache;
