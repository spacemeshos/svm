use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

use crate::memory::MemAppPages;

/// Initializes a new app pages backed by a new in-memory key-value store.
pub fn app_pages_init(
    addr: &str,
    pages_count: u16,
) -> (Address, Rc<RefCell<MemKVStore>>, MemAppPages) {
    let addr = Address::of(addr);
    let kv = Rc::new(RefCell::new(MemKVStore::new()));

    let pages = MemAppPages::new(addr.clone(), Rc::clone(&kv), State::empty(), pages_count);

    (addr, kv, pages)
}

/// Initializes a new app pages backed by an existing in-memory key-value store.
pub fn app_pages_open(
    addr: &Address,
    state: &State,
    kv: &Rc<RefCell<MemKVStore>>,
    pages_count: u16,
) -> MemAppPages {
    MemAppPages::new(addr.clone(), Rc::clone(&kv), state.clone(), pages_count)
}
