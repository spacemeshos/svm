use crate::page::{PageHash, PageIndex};
use crate::traits::{PageHasher, PagesStorage, StateAwarePagesStorage, StateHasher};

use svm_common::{Address, State};
use svm_kv::traits::KVStore;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use log::{debug, error, trace};

#[derive(Debug, Clone)]
enum PageEntry {
    Uninitialized,
    NotModified(PageHash),
    Modified(PageHash, Vec<u8>),
}

/// `ContractPages` is an implemetation of the `PagesStorage` trait that is state aware.
/// `KV` - stands for `KVStore`
/// `PH` - stands for `PageHasher`
/// `SH` - stands for `StateHasher`
pub struct ContractPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    state: State,
    addr: Address,
    pages: Vec<PageEntry>,
    kv: Rc<RefCell<KV>>,
    pages_count: u32,
    marker: PhantomData<(PH, SH)>,
}

impl<KV, PH, SH> ContractPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    /// Creates a new instance of `ContractPages`
    /// * `addr`        - The running contract account address.
    /// * `kv`          - The underlying kv-store used for retrieving a page raw-data when queried by its page-hash serving as a key.
    /// * `state`       - The current contract-storage state prior execution of the current contract transaction.
    /// * `pages_count` - The number of pages consumed by the contract storage (it's a fixed value per-contract).
    pub fn new(addr: Address, kv: Rc<RefCell<KV>>, state: State, pages_count: u32) -> Self {
        let mut storage = Self {
            state,
            kv,
            pages_count,
            addr,
            pages: vec![PageEntry::Uninitialized; pages_count as usize],
            marker: PhantomData,
        };

        storage.init_pages_state();

        storage
    }

    /// Loads the entry:
    /// state ---> [page1_hash || page2_hash || .... || pageN_hash]
    ///
    /// Then, populates `self.pages`. Each page is initialized with `PageEntry::NotModified(page_hash)`
    fn init_pages_state(&mut self) {
        debug!("initializating pages-storage with state {:?}", self.state);

        if self.state == State::empty() {
            // `self.state` is `000...0`. It means that state doesn't exist under the key-value store.
            // This happens when a Smart Contract runs for the first time.
            // We initialize each page with its zero-page hash `HASH(addr || page_idx || 0...0)`

            for page_idx in 0..(self.pages_count as usize) {
                let ph = self.compute_zero_page_hash(PageIndex(page_idx as u32));
                self.pages[page_idx] = PageEntry::NotModified(ph);
            }
        } else if let Some(v) = self.kv.borrow().get(self.state.as_slice()) {
            // `v` should be a concatenation of pages-hash. Each page hash consumes exactly 32 bytes.
            assert!(v.len() % 32 == 0);

            for (page_idx, raw_ph) in v.chunks_exact(32).enumerate() {
                let ph = PageHash::from(raw_ph);
                self.pages[page_idx] = PageEntry::NotModified(ph);

                trace!("page #{}, has page-hash {:?}", page_idx, ph);
            }
        } else {
            error!("Didn't find state: {:?}", self.state.as_slice());
            panic!("Didn't find state: {:?}", self.state.as_slice());
        }
    }

    /// Derives page hash, from its index `page_idx` and data `page_data`.
    #[must_use]
    #[inline(always)]
    pub fn compute_page_hash(&self, page_idx: PageIndex, page_data: &[u8]) -> PageHash {
        PH::hash(self.addr.clone(), page_idx, page_data)
    }

    /// Derives page hash for page indexed `page_idx` containing only zeros.
    #[must_use]
    #[inline(always)]
    pub fn compute_zero_page_hash(&self, page_idx: PageIndex) -> PageHash {
        let zeros_page = crate::page::zero_page();
        self.compute_page_hash(page_idx, zeros_page.as_ref())
    }

    /// The number of dirty pages
    pub fn dirty_pages_count(&self) -> usize {
        self.pages.iter().fold(0, |acc, page| match page {
            PageEntry::NotModified(..) => acc,
            PageEntry::Modified(..) => acc + 1,
            PageEntry::Uninitialized => unreachable!(),
        })
    }

    fn prepare_changeset(&self) -> (State, Vec<PageHash>, Vec<(&[u8], &[u8])>) {
        let mut changes = Vec::new();

        let mut pages_hash: Vec<PageHash> = Vec::new();

        for page in self.pages.iter() {
            match page {
                PageEntry::NotModified(ph) => pages_hash.push(*ph),
                PageEntry::Modified(ph, data) => {
                    let change: (&[u8], &[u8]) = (&ph.0, data);
                    changes.push(change);

                    pages_hash.push(*ph);
                }
                PageEntry::Uninitialized => unreachable!(),
            }
        }

        let new_state_hash = SH::hash(pages_hash.as_slice());
        let new_state = State::from(new_state_hash.as_ref());

        (new_state, pages_hash, changes)
    }
}

impl<KV, PH, SH> StateAwarePagesStorage for ContractPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    #[must_use]
    #[inline(always)]
    fn get_state(&self) -> State {
        self.state.clone()
    }

    #[must_use]
    fn get_page_hash(&self, page_idx: PageIndex) -> PageHash {
        match self.pages[page_idx.0 as usize] {
            PageEntry::NotModified(ph) => ph,
            PageEntry::Modified(ph, _) => ph,
            PageEntry::Uninitialized => unreachable!(),
        }
    }
}

impl<KV, PH, SH> PagesStorage for ContractPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    #[must_use]
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        match self.pages[page_idx.0 as usize] {
            PageEntry::NotModified(ph) => self.kv.borrow().get(&ph.0),
            PageEntry::Modified(..) => panic!("Not allowed to read a dirty page"),
            PageEntry::Uninitialized => unreachable!(),
        }
    }

    fn write_page(&mut self, page_idx: PageIndex, page_data: &[u8]) {
        let ph = self.compute_page_hash(page_idx, page_data);

        self.pages[page_idx.0 as usize] = PageEntry::Modified(ph, page_data.to_vec());
    }

    fn clear(&mut self) {
        debug!("clearing pages-storage...");

        for page in &mut self.pages {
            match page {
                PageEntry::Modified(ph, ..) => *page = PageEntry::NotModified(*ph),
                PageEntry::NotModified(..) => (),
                PageEntry::Uninitialized => unreachable!(),
            }
        }
    }

    fn commit(&mut self) {
        // We have each page-hash (dirty and non-dirty) under `self.pages`
        // Now, we'll compute the new state of the Smart Contract pages.
        //
        // ```
        // new_state = HASH(page1_hash || page2_hash || ... || pageN_hash)
        // ```

        debug!("about to commit dirty pages to underlying key-value store");

        let (new_state, pages_hash, changeset) = self.prepare_changeset();

        let mut entries: Vec<(&[u8], &[u8])> = Vec::with_capacity(1 + changeset.len());

        let state_entry_val: Vec<u8> = pages_hash.iter().flat_map(|ph| ph.0.to_vec()).collect();
        entries.push((new_state.as_slice(), state_entry_val.as_ref()));

        for change in changeset {
            entries.push(change)
        }

        // At last, we store under the flat key-value store (`self.kv`) the following new entries:
        // ```
        // new_state  ---> [page1_hash, page2_hash, ..., pageN_hash]
        // page1_hash ---> page1_content
        // page2_hash ---> page2_content
        // ...
        // ...
        // pageN_hash ---> pageN_content
        // ```

        self.kv.borrow_mut().store(entries.as_slice());
        self.state = new_state;

        self.clear();
    }
}

impl<KV, PH, SH> Drop for ContractPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    fn drop(&mut self) {
        debug!("dropping `ContractPages`...");
    }
}
