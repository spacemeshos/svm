use std::cell::RefCell;
use std::rc::Rc;

use svm_common::Address;

use crate::app::AppKVStore;
use crate::kv::{FakeKV, StatefulKVStore};

/// Creates an in-memory stateful key-value store and returns it wrapped within `Rc<RefCell<..>>`
pub fn create_raw_kv() -> Rc<RefCell<dyn StatefulKVStore>> {
    Rc::new(RefCell::new(FakeKV::new()))
}

/// Creates an `AppKVStore` for app having `Address = app_addr`.
///
/// The underlying raw key-value store is in-memory (see `create_raw_kv`).
pub fn create_app_kv(app_addr: Address) -> AppKVStore {
    let raw_kv = create_raw_kv();

    AppKVStore::new(app_addr, &raw_kv)
}
