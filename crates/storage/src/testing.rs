//! Tests helpers

use std::cell::RefCell;
use std::rc::Rc;

use svm_types::Address;

use crate::account::AccountKVStore;
use crate::kv::{FakeKV, StatefulKV};

/// Creates an in-memory stateful key-value store and returns it wrapped within `Rc<RefCell<..>>`
pub fn create_kv() -> Rc<RefCell<dyn StatefulKV>> {
    Rc::new(RefCell::new(FakeKV::new()))
}

/// Creates an [`AccountKVStore`] for an `Account` having `Address` equals `account_addr`.
///
/// The underlying raw key-value store is in-memory (see `create_raw_kv`).
pub fn create_account_kv(account_addr: Address) -> AccountKVStore {
    let kv = create_kv();

    AccountKVStore::new(account_addr, &kv)
}
