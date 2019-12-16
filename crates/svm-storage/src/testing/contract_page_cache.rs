use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

use crate::memory::MemContractPages;

use crate::default::DefaultPageCache;
use crate::testing;

pub fn contract_page_cache_init(
    addr: u32,
    pages_count: u32,
) -> (
    Address,
    Rc<RefCell<MemKVStore>>,
    DefaultPageCache<MemContractPages>,
) {
    let (addr, kv, pages) = testing::contract_pages_init(addr, pages_count);

    let cache = DefaultPageCache::new(pages, pages_count);

    (addr, kv, cache)
}

pub fn contract_page_cache_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u32,
) -> DefaultPageCache<MemContractPages> {
    let pages = testing::contract_pages_open(addr, state, kv, pages_count);

    DefaultPageCache::new(pages, pages_count)
}
