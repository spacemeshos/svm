use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, DefaultKeyHasher, KeyHasher};
use svm_kv::traits::KVStore;

pub struct AppKVStore {
    pub(crate) app_addr: Address,

    pub(crate) raw_kv: Rc<RefCell<dyn KVStore>>,
}

impl KVStore for AppKVStore {
    #[must_use]
    #[inline]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        debug_assert_eq!(key.len(), 4);

        let key = self.build_key(key);

        self.get(&key)
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        let changes: Vec<_> = changes
            .iter()
            .map(|(k, v)| {
                let k = self.build_key(k);

                (k, *v)
            })
            .collect();

        let changes: Vec<_> = changes.iter().map(|(k, v)| (&k[..], *v)).collect();

        self.raw_kv.borrow_mut().store(&changes);
    }
}

impl AppKVStore {
    pub fn new(app_addr: Address, raw_kv: Rc<RefCell<dyn KVStore>>) -> Self {
        Self { app_addr, raw_kv }
    }

    #[inline]
    fn build_key(&self, key: &[u8]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Address::len() + key.len());

        buf.extend_from_slice(self.app_addr.as_slice());
        buf.extend_from_slice(key);

        self.hash(&buf)
    }

    #[inline]
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        DefaultKeyHasher::hash(bytes).to_vec()
    }
}
