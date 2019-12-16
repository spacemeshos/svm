use std::cell::RefCell;
use std::rc::Rc;

use crate::{testing, ContractStorage};

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

pub fn contract_storage_init(
    addr: u32,
    pages_count: u32,
) -> (Address, Rc<RefCell<MemKVStore>>, ContractStorage) {
    let (addr, kv, cache) = testing::contract_page_cache_init(addr, pages_count);

    let storage = ContractStorage::new(Box::new(cache));

    (addr, kv, storage)
}

pub fn contract_storage_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u32,
) -> ContractStorage {
    let cache = testing::contract_page_cache_open(addr, state, kv, pages_count);

    ContractStorage::new(Box::new(cache))
}
