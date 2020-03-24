use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::{
    page::{PageAddr, PageIndex},
    traits::{PageAddrHasher, PagesStorage},
};

use svm_common::Address;
use svm_kv::traits::KVStore;

/// `DefaultPagesStorage` is the default implementation for the `PagesStorage` trait.
/// It serves as a wrapper to a key-value store.
///
/// * When we do `read_page` we take the input page (`u32`), compute its hash (a.k.a `page-key`)
///   and do a lookup on the wrapped key-value store.
///   Similarly, when we do `write_page`, we take the input page (`u32`), compute its hash (a.k.a `page-key`)
///   and insert the new `page-key -> data (of type &[u8])` into the `uncommitted` standard Rust `HashMap`.
///
/// * `DefaultPagesStorage` doesn't deal with caching at all. During execution of an app
///    we are supposed to use a `PageCache` the wraps the `DefaultPagesStorage` (or other `PagesStorage`).
///    Given that, the `DefaultPagesStorage` is meant to read each page at most once per running-app.
///    (i.e when the wrapping `PageCache` is having a cache miss).
///
/// * As described above, calling `write_page` data isn't being persisted to the key-value store.
///   But it will await to a future `commit`. This is by design since an app execution
///   may fail for multiple reasons, and on such occurrence we don't want to change any state.
///   Another benefit is that if the underlying key-value store supports a batch write (for example
///   database `rocksdb` has this capability), the `commit` implementation can take advantage of it.
pub struct DefaultPagesStorage<PAH: PageAddrHasher, KV: KVStore> {
    app_addr: Address,
    kv: Rc<RefCell<KV>>,
    phantom: PhantomData<PAH>,
    uncommitted: HashMap<PageAddr, Vec<u8>>,
}

impl<PAH, KV> DefaultPagesStorage<PAH, KV>
where
    PAH: PageAddrHasher,
    KV: KVStore,
{
    /// Creates a new `DefaultPagesStorage`
    #[allow(unused)]
    pub fn new(app_addr: Address, kv: Rc<RefCell<KV>>) -> Self {
        Self {
            app_addr,
            kv,
            uncommitted: HashMap::new(),
            phantom: PhantomData,
        }
    }

    #[must_use]
    #[inline]
    fn compute_page_addr(&self, page_idx: PageIndex) -> PageAddr {
        PAH::hash(&self.app_addr, page_idx)
    }
}

impl<PAH, KV> PagesStorage for DefaultPagesStorage<PAH, KV>
where
    PAH: PageAddrHasher,
    KV: KVStore,
{
    /// We assume that the `page` has no pending changes (see more detailed explanation above).
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        let page_addr = self.compute_page_addr(page_idx);
        let key = page_addr.inner().as_slice();

        self.kv.borrow().get(&key)
    }

    /// Pushes a new pending change (persistence *only* upon `commit`)
    fn write_page(&mut self, page_idx: PageIndex, data: &[u8]) {
        let page_addr = self.compute_page_addr(page_idx);

        self.uncommitted.insert(page_addr, data.to_vec());
    }

    /// Clears the pending channges
    fn clear(&mut self) {
        self.uncommitted.clear();
    }

    /// Commits pending changes to the underlying key-value store
    fn commit(&mut self) {
        let changes: Vec<_> = self
            .uncommitted
            .iter()
            .map(|(page_addr, data)| {
                let page_addr = page_addr.inner().as_slice();
                (page_addr, &data[..])
            })
            .collect();

        self.kv.borrow_mut().store(&changes[..]);

        self.clear();
    }
}
