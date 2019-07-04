mod cacheable_pages_storage;
mod default_page_hasher;
mod mem_kv_store;
mod mem_pages_storage;
mod pages_storage_impl;
mod traits;

use cacheable_pages_storage::CacheablePagesStorage;
use default_page_hasher::DefaultPageHasher;
use mem_kv_store::MemKVStore;
use mem_pages_storage::MemPagesStorage;
use pages_storage_impl::PagesStorageImpl;
