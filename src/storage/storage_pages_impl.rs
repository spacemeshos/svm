use super::traits::{KVStore, StoragePageHasher, StoragePages};
use crate::common::Address;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

type PageKey = [u8; 32];

pub struct StoragePagesImpl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> {
    contract_addr: Address,
    kv_store: Rc<RefCell<KV>>,
    ph_marker: PhantomData<SPH>,
}

impl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> StoragePagesImpl<SPH, KV> {
    pub fn new(contract_addr: Address, kv_store: Rc<RefCell<KV>>) -> Self {
        Self {
            contract_addr,
            kv_store,
            ph_marker: PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn compute_page_hash(&self, page: u32) -> [u8; 32] {
        SPH::hash(self.contract_addr, page)
    }
}

impl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> StoragePages for StoragePagesImpl<SPH, KV> {
    fn read_page(&self, page: u32) -> Vec<u8> {
        let ph = self.compute_page_hash(page);

        self.kv_store.borrow().get(ph)
    }

    fn write_page(&mut self, page: u32, data: &[u8]) {
        let ph = self.compute_page_hash(page);

        self.kv_store.borrow_mut().store(ph, data);
    }
}
