use std::cell::RefCell;
use std::rc::Rc;

use svm_common::Address;
use svm_kv::traits::KVStore;

pub struct AppKVStore {
    app_addr: Address,

    raw_kv: Rc<RefCell<dyn KVStore>>,
}

impl KVStore for AppKVStore {
    #[must_use]
    #[inline]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key = self.build_key(key);

        self.get(&key)
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        todo!()
    }
}

impl AppKVStore {
    pub fn new(app_addr: Address, raw_kv: Rc<RefCell<dyn KVStore>>) -> Self {
        Self { app_addr, raw_kv }
    }

    fn build_key(&self, key: &[u8]) -> Vec<u8> {
        todo!()
    }
}
