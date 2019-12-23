use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

use crate::memory::MemContractPages;

/// Initializes a new contract pages backed by a new in-memory key-value store.
pub fn contract_pages_init(
    addr: u32,
    pages_count: u32,
) -> (Address, Rc<RefCell<MemKVStore>>, MemContractPages) {
    let addr = Address::from(addr as u32);
    let kv = Rc::new(RefCell::new(MemKVStore::new()));

    let pages = MemContractPages::new(addr.clone(), Rc::clone(&kv), State::empty(), pages_count);

    (addr, kv, pages)
}

/// Initializes a new contract pages backed by an existing in-memory key-value store.
pub fn contract_pages_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u32,
) -> MemContractPages {
    MemContractPages::new(addr.clone(), Rc::clone(&kv), state.clone(), pages_count)
}
