use super::traits::{KVStore, StoragePageHasher, StoragePages};
use super::MemKVStore;
use crate::common::Address;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

type PageKey = [u8; 32];

pub struct StoragePagesImpl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> {
    contract_addr: Address,
    db: Rc<RefCell<KV>>,
    ph_marker: PhantomData<SPH>,
}

impl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> StoragePagesImpl<SPH, KV> {
    pub fn new(contract_addr: Address, db: Rc<RefCell<KV>>) -> Self {
        Self {
            contract_addr,
            db,
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
    fn read_page(&mut self, page: u32) -> Option<Vec<u8>> {
        let ph = self.compute_page_hash(page);

        self.db.borrow().get(ph)
    }

    fn write_page(&mut self, page: u32, data: &[u8]) {
        let ph = self.compute_page_hash(page);

        self.db.borrow_mut().store(ph, data);
    }

    fn clear(&mut self) {}

    fn commit(&mut self) {}
}
