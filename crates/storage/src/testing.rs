//! Tests helpers

use std::sync::{Arc, Mutex};

use svm_types::Address;

use crate::account::AccountKVStore;
use crate::kv::{FakeKV, StatefulKV};

/// Creates an in-memory stateful key-value store and returns it wrapped within `Rc<RefCell<..>>`
pub fn create_kv() -> Arc<Mutex<dyn StatefulKV + Send>> {
    Arc::new(Mutex::new(FakeKV::new()))
}

/// Creates an [`AccountKVStore`] for an `Account` having `Address` equals `account_addr`.
///
/// The underlying raw key-value store is in-memory (see `create_raw_kv`).
pub fn create_account_kv(account_addr: Address) -> AccountKVStore {
    let kv = create_kv();

    AccountKVStore::new(account_addr, &kv)
}
