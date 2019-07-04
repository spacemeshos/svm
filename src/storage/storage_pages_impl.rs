use super::traits::{KVStore, PageHasher, StoragePages};
use super::MemKVStore;
use crate::common::Address;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

type PageKey = [u8; 32];

pub struct StoragePagesImpl<PH: PageHasher, KV: KVStore<K = PageKey>> {
    contract_addr: Address,
    db: Rc<RefCell<KV>>,
    uncommitted: MemKVStore<PageKey>,
    ph_marker: PhantomData<PH>,
}

impl<PH: PageHasher, KV: KVStore<K = PageKey>> StoragePagesImpl<PH, KV> {
    pub fn new(contract_addr: Address, db: Rc<RefCell<KV>>) -> Self {
        Self {
            contract_addr,
            db,
            uncommitted: MemKVStore::new(),
            ph_marker: PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn compute_page_hash(&self, page: u32) -> [u8; 32] {
        PH::hash(self.contract_addr, page)
    }

    #[cfg(test)]
    pub fn uncommitted_len(&self) -> usize {
        self.uncommitted.map.len()
    }
}

impl<PH: PageHasher, KV: KVStore<K = PageKey>> StoragePages for StoragePagesImpl<PH, KV> {
    fn read_page(&mut self, page_idx: u32) -> Option<Vec<u8>> {
        let ph = self.compute_page_hash(page_idx);

        let page = self.uncommitted.get(ph);

        if page.is_some() {
            return page;
        } else {
            // TODO: make sure we don't recompute `ph`
            let ph = self.compute_page_hash(page_idx);

            self.db.borrow().get(ph)
        }
    }

    fn write_page(&mut self, page_idx: u32, data: &[u8]) {
        let ph = self.compute_page_hash(page_idx);

        self.uncommitted.store(ph, data);
    }

    fn clear(&mut self) {
        self.uncommitted.clear();
    }

    fn commit(&mut self) {
        for (key, page) in &self.uncommitted.map {
            self.db.borrow_mut().store(key.to_owned(), page);
        }

        self.clear();
    }
}
