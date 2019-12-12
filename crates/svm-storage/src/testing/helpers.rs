use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, DefaultKeyHasher, KeyHasher, State};
use svm_kv::memory::MemKVStore;

use crate::default::{DefaultPageCache, DefaultPageHasher, DefaultPageIndexHasher};
use crate::memory::MemContractPages;
use crate::page::{PageHash, PageIndex};
use crate::traits::{PageHasher, PageIndexHasher};
use crate::ContractStorage;

pub fn concat_pages_hash(pages_hash: &[PageHash]) -> Vec<u8> {
    let mut res = Vec::new();

    for ph in pages_hash.iter() {
        res.extend_from_slice(ph.as_ref());
    }

    res
}

pub fn compute_pages_state(pages_hash: &[PageHash]) -> State {
    let concat_ph = concat_pages_hash(pages_hash);

    let state = Some(concat_ph.as_slice())
        .map(|jph| {
            let h = DefaultKeyHasher::hash(jph);
            State::from(h.as_ref())
        })
        .unwrap();

    state
}

pub fn contract_pages_init(
    addr: u32,
    pages_count: u32,
) -> (Address, Rc<RefCell<MemKVStore>>, MemContractPages) {
    let addr = Address::from(addr as u32);
    let kv = Rc::new(RefCell::new(MemKVStore::new()));

    let pages = MemContractPages::new(addr.clone(), Rc::clone(&kv), State::empty(), pages_count);

    (addr, kv, pages)
}

pub fn contract_pages_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u32,
) -> MemContractPages {
    MemContractPages::new(addr.clone(), Rc::clone(&kv), state.clone(), pages_count)
}

pub fn contract_page_cache_init(
    addr: u32,
    pages_count: u32,
) -> (
    Address,
    Rc<RefCell<MemKVStore>>,
    DefaultPageCache<MemContractPages>,
) {
    let (addr, kv, pages) = contract_pages_init(addr, pages_count);

    let cache = DefaultPageCache::new(pages, pages_count as usize);

    (addr, kv, cache)
}

pub fn contract_page_cache_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u32,
) -> DefaultPageCache<MemContractPages> {
    let pages = contract_pages_open(addr, state, kv, pages_count);

    DefaultPageCache::new(pages, pages_count as usize)
}

pub fn contract_storage_init(
    addr: u32,
    pages_count: u32,
) -> (Address, Rc<RefCell<MemKVStore>>, ContractStorage) {
    let (addr, kv, cache) = contract_page_cache_init(addr, pages_count);

    let storage = ContractStorage::new(Box::new(cache));

    (addr, kv, storage)
}

pub fn contract_storage_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u32,
) -> ContractStorage {
    let cache = contract_page_cache_open(addr, state, kv, pages_count);

    ContractStorage::new(Box::new(cache))
}

/// An helper for computing a page default hash using `DefaultPageIndexHasher`
pub fn default_page_hash(addr: &Address, page_idx: u32, data: &[u8]) -> PageHash {
    DefaultPageHasher::hash(addr.clone(), PageIndex(page_idx), data)
}

/// An helper for computing page-index hashes using `DefaultPageIndexHasher`
pub fn default_page_index_hash(addr: u32, page_idx: u32) -> [u8; 32] {
    let addr = Address::from(addr as u32);

    DefaultPageIndexHasher::hash(addr, PageIndex(page_idx))
}

pub fn fill_page(page: &mut [u8], items: &[(usize, u8)]) {
    for (i, b) in items {
        page[*i] = *b;
    }
}
