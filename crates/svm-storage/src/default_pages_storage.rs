use super::traits::{KVStore, PageHasher, PagesStorage};
use super::MemKVStore;
use svm_common::Address;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// `DefaultPagesStorage` assume that the `PageHasher` computes a 32 bytes hashes
type PageKey = [u8; 32];

/// `DefaultPagesStorage` is the default implementation for the `PagesStorage` trait.
/// It serves as a wrapper to a key-value store.
///
/// * When we do `read_page` we take the input page (`u32`), compute its hash (a.k.a `page-key`)
///   and do a lookup on the wrapped key-value store.
///   Similarly, when we do `write_page`, we take the input page (`u32`), compute its hash (a.k.a `page-key`)
///   and insert the new `page-key -> data (of type &[u8])` into the `uncommitted` `MemKVStore` (wraps a `HashMap`).
///
/// * For Smart Contracts we use a Trie based key-value store. However `DefaultPagesStorage` is ignorant
///   of the actual key-value store being used.
///
/// * `DefaultPagesStorage` doesn't deal with caching at all. During execution of a Smart Contract
///    we are supposed to use a `PageCache` the wraps the `DefaultPagesStorage` (or other `PagesStorage`).
///    Given that, the `DefaultPagesStorage` is meant to read each page at most once per a Smart Contract running
///    (i.e when the wrapping `PageCache` is having a cache miss).
///
/// * As described above, calling `write_page` data isn't being persisted to the key-value store.
///   But it will await to a future `commit`. This is by design since a Smart Contract execution
///   may fail for multiple reasons, and on such occurrence we don't want to change any state.
///   Another benefit is that if the underlying key-value store supports a batch write (for example
///   databases `leveldb` and `rocksdb` have this capability), the `commit` implementation can take advantage of it.
pub struct DefaultPagesStorage<PH: PageHasher, KV: KVStore<K = PageKey>> {
    contract_addr: Address,
    db: Rc<RefCell<KV>>,
    uncommitted: MemKVStore<PageKey>,
    ph_marker: PhantomData<PH>,
}

impl<PH: PageHasher, KV: KVStore<K = PageKey>> DefaultPagesStorage<PH, KV> {
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

impl<PH: PageHasher, KV: KVStore<K = PageKey>> PagesStorage for DefaultPagesStorage<PH, KV> {
    /// We assume that the `page` has no pending changes (see more detailed explanation above).
    fn read_page(&mut self, page_idx: u32) -> Option<Vec<u8>> {
        let ph = self.compute_page_hash(page_idx);

        self.db.borrow().get(ph)
    }

    /// Pushes a new pending change (persistence *only* upon `commit`)
    fn write_page(&mut self, page_idx: u32, data: &[u8]) {
        let ph = self.compute_page_hash(page_idx);

        self.uncommitted.store(ph, data);
    }

    /// Clears the pending channges
    fn clear(&mut self) {
        self.uncommitted.clear();
    }

    /// Commits pending changes to the underlying key-value store
    fn commit(&mut self) {
        for (key, page) in &self.uncommitted.map {
            self.db.borrow_mut().store(key.to_owned(), page);
        }

        self.clear();
    }
}
