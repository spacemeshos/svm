use crate::page::PageIndex;
use crate::traits::{KVStore, PageIndexHasher, PagesStorage};

use svm_common::Address;

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

/// `DefaultPagesStorage` assume that the `PageIndexHasher` computes a 32 bytes hashes
type PageKey = [u8; 32];

/// `DefaultPagesStorage` is the default implementation for the `PagesStorage` trait.
/// It serves as a wrapper to a key-value store.
///
/// * When we do `read_page` we take the input page (`u32`), compute its hash (a.k.a `page-key`)
///   and do a lookup on the wrapped key-value store.
///   Similarly, when we do `write_page`, we take the input page (`u32`), compute its hash (a.k.a `page-key`)
///   and insert the new `page-key -> data (of type &[u8])` into the `uncommitted` standard Rust `HashMap`.
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
pub struct DefaultPagesStorage<PH: PageIndexHasher, KV: KVStore<K = PageKey>> {
    addr: Address,
    db: Rc<RefCell<KV>>,
    uncommitted: HashMap<PageKey, Vec<u8>>,
    marker: PhantomData<PH>,
}

impl<PH: PageIndexHasher, KV: KVStore<K = PageKey>> DefaultPagesStorage<PH, KV> {
    /// Creates a new `DefaultPagesStorage`
    pub fn new(addr: Address, db: Rc<RefCell<KV>>) -> Self {
        Self {
            addr,
            db,
            uncommitted: HashMap::new(),
            marker: PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    fn compute_page_hash(&self, page_idx: PageIndex) -> [u8; 32] {
        PH::hash(self.addr, page_idx)
    }

    #[cfg(test)]
    pub fn uncommitted_len(&self) -> usize {
        self.uncommitted.len()
    }
}

impl<PH: PageIndexHasher, KV: KVStore<K = PageKey>> PagesStorage for DefaultPagesStorage<PH, KV> {
    /// We assume that the `page` has no pending changes (see more detailed explanation above).
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        let ph = self.compute_page_hash(page_idx);

        self.db.borrow().get(ph)
    }

    /// Pushes a new pending change (persistence *only* upon `commit`)
    fn write_page(&mut self, page_idx: PageIndex, data: &[u8]) {
        let ph = self.compute_page_hash(page_idx);

        self.uncommitted.insert(ph, data.to_vec());
    }

    /// Clears the pending channges
    fn clear(&mut self) {
        self.uncommitted.clear();
    }

    /// Commits pending changes to the underlying key-value store
    fn commit(&mut self) {
        let changes: Vec<(PageKey, &[u8])> = self
            .uncommitted
            .iter()
            .map(|(key, page)| (key.to_owned(), page.as_slice()))
            .collect();

        self.db.borrow_mut().store(changes.as_slice());

        self.clear();
    }
}
