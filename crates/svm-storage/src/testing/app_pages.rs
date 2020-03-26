use std::{cell::RefCell, rc::Rc};

use svm_common::State;
use svm_kv::memory::MemKVStore;

use crate::memory::MemAppPages;

/// Initializes a new app pages backed by a new in-memory key-value store.
pub fn app_pages_init(page_count: u16) -> (Rc<RefCell<MemKVStore>>, MemAppPages) {
    let kv = Rc::new(RefCell::new(MemKVStore::new()));

    let pages = MemAppPages::new(Rc::clone(&kv), State::empty(), page_count);

    (kv, pages)
}

/// Initializes a new app pages backed by an existing in-memory key-value store.
pub fn app_pages_open(state: &State, kv: &Rc<RefCell<MemKVStore>>, page_count: u16) -> MemAppPages {
    MemAppPages::new(Rc::clone(&kv), state.clone(), page_count)
}
