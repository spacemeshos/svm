use std::{cell::RefCell, rc::Rc};

use svm_common::State;
use svm_kv::memory::MemKVStore;

use crate::{default::DefaultPageCache, memory::MemAppPages, testing};

/// Initialises a new page-cache backed by a new initialized in-memory pages-storage.
pub fn app_page_cache_init(
    page_count: u16,
) -> (Rc<RefCell<MemKVStore>>, DefaultPageCache<MemAppPages>) {
    let (kv, pages) = testing::app_pages_init(page_count);

    let cache = DefaultPageCache::new(pages, page_count);

    (kv, cache)
}

/// Initialises a new page-cache backed by an existing in-memory pages-storage.
pub fn app_page_cache_open(
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    page_count: u16,
) -> DefaultPageCache<MemAppPages> {
    let pages = testing::app_pages_open(state, kv, page_count);

    DefaultPageCache::new(pages, page_count)
}
