use crate::page::PageIndex;
use crate::traits::{PageIndexHasher, PagesStorage};
use svm_kv::traits::KVStore;

use svm_common::Address;

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

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
pub struct DefaultPagesStorage<PIH: PageIndexHasher, KV: KVStore> {
    addr: Address,
    kv: Rc<RefCell<KV>>,
    uncommitted: HashMap<Vec<u8>, Vec<u8>>,
    marker: PhantomData<PIH>,
}

impl<PIH, KV> DefaultPagesStorage<PIH, KV>
where
    PIH: PageIndexHasher,
    KV: KVStore,
{
    /// Creates a new `DefaultPagesStorage`
    #[allow(unused)]
    pub fn new(addr: Address, kv: Rc<RefCell<KV>>) -> Self {
        Self {
            addr,
            kv,
            uncommitted: HashMap::new(),
            marker: PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    fn compute_page_hash(&self, page_idx: PageIndex) -> [u8; 32] {
        PIH::hash(self.addr.clone(), page_idx)
    }
}

impl<PIH, KV> PagesStorage for DefaultPagesStorage<PIH, KV>
where
    PIH: PageIndexHasher,
    KV: KVStore,
{
    /// We assume that the `page` has no pending changes (see more detailed explanation above).
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        let ph = self.compute_page_hash(page_idx);

        self.kv.borrow().get(&ph)
    }

    /// Pushes a new pending change (persistence *only* upon `commit`)
    fn write_page(&mut self, page_idx: PageIndex, data: &[u8]) {
        let ph = self.compute_page_hash(page_idx);

        self.uncommitted.insert(ph.to_vec(), data.to_vec());
    }

    /// Clears the pending channges
    fn clear(&mut self) {
        self.uncommitted.clear();
    }

    /// Commits pending changes to the underlying key-value store
    fn commit(&mut self) {
        let changes: Vec<(&[u8], &[u8])> = self
            .uncommitted
            .iter()
            .map(|(key, page)| (key.as_ref(), page.as_ref()))
            .collect();

        self.kv.borrow_mut().store(changes.as_slice());

        self.clear();
    }
}
