use std::cell::RefCell;
use std::rc::Rc;

use crate::{testing, AppStorage};

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

/// Initialises a new `AppStorage` derived its address and #pages and empty state (`00...0`)
pub fn app_storage_init(
    addr: u32,
    pages_count: u16,
) -> (Address, Rc<RefCell<MemKVStore>>, AppStorage) {
    let (addr, kv, cache) = testing::app_page_cache_init(addr, pages_count);

    let storage = AppStorage::new(Box::new(cache));

    (addr, kv, storage)
}

/// Initialises a new `AppStorage` derived its address, state and #pages.
/// Storage is backed by an key-value store `kv`
pub fn app_storage_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u16,
) -> AppStorage {
    let cache = testing::app_page_cache_open(addr, state, kv, pages_count);

    AppStorage::new(Box::new(cache))
}
