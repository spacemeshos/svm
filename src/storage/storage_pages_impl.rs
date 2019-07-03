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
    uncommitted: MemKVStore<PageKey>,
    ph_marker: PhantomData<SPH>,
}

impl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> StoragePagesImpl<SPH, KV> {
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
        SPH::hash(self.contract_addr, page)
    }

    #[cfg(test)]
    pub fn uncommitted_len(&self) -> usize {
        self.uncommitted.map.len()
    }
}

impl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> StoragePages for StoragePagesImpl<SPH, KV> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::DefaultPageHasher;
    use crate::Address;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type MemStoragePages<K> = StoragePagesImpl<DefaultPageHasher, MemKVStore<K>>;

    #[test]
    fn a_page_does_not_exit_by_default() {
        let addr = Address::from(0x11_22_33_44 as u32);

        let mut kv = Rc::new(RefCell::new(MemKVStore::new()));
        let mut storage = MemStoragePages::new(addr, kv);

        assert_eq!(None, storage.read_page(0));
    }

    #[test]
    fn writing_a_page_does_not_auto_commit_it_to_underlying_kv() {
        let addr = Address::from(0x11_22_33_44 as u32);

        let mut kv = Rc::new(RefCell::new(MemKVStore::new()));
        let mut kv_clone = Rc::clone(&kv);

        // both `storage1` and `storage2` service the same contract address `addr`
        // and both share the the same underlying key-value store
        let mut storage1 = MemStoragePages::new(addr, kv);
        let mut storage2 = MemStoragePages::new(addr, kv_clone);

        // writing `page 0` with data `[10, 20, 30]`
        // changes aren't commited directly to `kv`
        // storage `storage1` saves the a pending commit page,
        // storage `storage2` won't see that changes before `storage1` doing `commit`
        storage1.write_page(0, &vec![10, 20, 30]);
        assert_eq!(vec![10, 20, 30], storage1.read_page(0).unwrap());
        assert_eq!(None, storage2.read_page(0));

        // another assertion for the uncommitted changes
        assert_eq!(1, storage1.uncommitted_len());
        assert_eq!(0, storage2.uncommitted_len());

        // now, storage `storage1` commits pending changes to `kv`
        storage1.commit();

        // both `storage1` and `storage2` report the same persisted `page 0`
        assert_eq!(vec![10, 20, 30], storage1.read_page(0).unwrap());
        assert_eq!(vec![10, 20, 30], storage2.read_page(0).unwrap());

        // no more pending changes
        assert_eq!(0, storage1.uncommitted_len());
        assert_eq!(0, storage2.uncommitted_len());
    }

    #[test]
    fn writing_the_same_page_twice_before_committing() {
        let addr = Address::from(0x11_22_33_44 as u32);

        let mut kv = Rc::new(RefCell::new(MemKVStore::new()));
        let mut storage = MemStoragePages::new(addr, kv);

        // first write
        storage.write_page(0, &vec![10, 20, 30]);
        assert_eq!(vec![10, 20, 30], storage.read_page(0).unwrap());
        // one pending change
        assert_eq!(1, storage.uncommitted_len());

        // page-override
        storage.write_page(0, &vec![40, 50, 60]);
        assert_eq!(vec![40, 50, 60], storage.read_page(0).unwrap());
        // still, one pending change
        assert_eq!(1, storage.uncommitted_len());

        // commit page
        storage.commit();

        assert_eq!(vec![40, 50, 60], storage.read_page(0).unwrap());
        // no pending change
        assert_eq!(0, storage.uncommitted_len());
    }

    #[test]
    fn committing_the_same_page_under_two_different_contract_addresses() {
        let addr1 = Address::from(0x11_22_33_44 as u32);
        let addr2 = Address::from(0x55_66_77_88 as u32);

        let mut kv = Rc::new(RefCell::new(MemKVStore::new()));
        let mut kv_clone = Rc::clone(&kv);

        // `storagee1` and `storage2` share the same underlying `kv store`
        let mut storage1 = MemStoragePages::new(addr1, kv);
        let mut storage2 = MemStoragePages::new(addr2, kv_clone);

        storage1.write_page(0, &vec![10, 20, 30]);
        storage2.write_page(0, &vec![40, 50, 60]);

        // committing pending changes
        storage1.commit();
        storage2.commit();

        // no more pending changes
        assert_eq!(0, storage1.uncommitted_len());
        assert_eq!(0, storage2.uncommitted_len());

        // two pages `[10, 20, 30]` and `[40, 50, 60]` have been committed successfully
        assert_eq!(vec![10, 20, 30], storage1.read_page(0).unwrap());
        assert_eq!(vec![40, 50, 60], storage2.read_page(0).unwrap());
    }
}
