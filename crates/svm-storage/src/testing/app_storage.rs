use std::{cell::RefCell, rc::Rc};

use crate::{testing, AppStorage};

use svm_common::State;
use svm_kv::memory::MemKVStore;

/// Initialises a new `AppStorage` derived its address and #pages and empty state (`00...0`)
pub fn app_storage_init(page_count: u16) -> (Rc<RefCell<MemKVStore>>, AppStorage) {
    let (kv, cache) = testing::app_page_cache_init(page_count);

    let storage = AppStorage::new(Box::new(cache));

    (kv, storage)
}

/// Initialises a new `AppStorage` derived its address, state and #pages.
/// Storage is backed by an key-value store `kv`
pub fn app_storage_open(
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    page_count: u16,
) -> AppStorage {
    let cache = testing::app_page_cache_open(state, kv, page_count);

    AppStorage::new(Box::new(cache))
}
