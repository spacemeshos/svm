use crate::{
    page::{JoinedPagesHash, PageHash, PageIndex},
    traits::{PageHasher, PagesStorage, StateAwarePagesStorage, StateHasher},
};

use svm_common::{Address, State};
use svm_kv::traits::KVStore;

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use log::{debug, error, trace};

#[derive(Debug, Clone)]
enum PageEntry {
    Uninitialized,
    NotModified(PageHash),
    Modified(PageHash, Vec<u8>),
}

#[derive(Debug, PartialEq, Eq)]
struct PageChange {
    idx: PageIndex,
    new_hash: PageHash,
    new_data: Vec<u8>,
}

struct Changeset {
    state: State,
    changes: Vec<PageChange>,
    jph: JoinedPagesHash,
}

/// `AppPages` is an implemetation of the `PagesStorage` trait that is `State`-aware.
///
/// `KV` - stands for `KVStore`
/// `PH` - stands for `PageHasher`
/// `SH` - stands for `StateHasher`
pub struct AppPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    state: State,
    app_addr: Address,
    pages: Vec<PageEntry>,
    kv: Rc<RefCell<KV>>,
    page_count: usize,
    phantom: PhantomData<(PH, SH)>,
}

impl<KV, PH, SH> AppPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    /// Creates a new instance of `AppPages`
    /// * `app_addr`    - The running app account address.
    /// * `kv`          - The underlying kv-store used for retrieving a page raw-data when queried by its page-hash serving as a key.
    /// * `state`       - The current app-storage state prior execution of the current app-transaction.
    /// * `page_count` - The number of pages consumed by the app-storage (it's a fixed value per-app).
    pub fn new(app_addr: Address, kv: Rc<RefCell<KV>>, state: State, page_count: u16) -> Self {
        let mut storage = Self {
            state,
            kv,
            page_count: page_count as usize,
            app_addr,
            pages: vec![PageEntry::Uninitialized; page_count as usize],
            phantom: PhantomData,
        };

        if storage.state == State::empty() {
            storage.init_state();
        } else {
            storage.load_pages_hash();
        }

        storage
    }

    fn init_state(&mut self) {
        // `self.state` is `000...0`. It means that state doesn't exist under the key-value store.
        // This happens when an app runs for the first time.
        // We initialize each page with its zero-page hash `HASH(addr || page_idx || 0...0)`

        let zph = self.compute_zero_page_hash();

        for page_idx in 0..self.page_count {
            self.pages[page_idx] = PageEntry::NotModified(zph.clone());
        }
    }

    fn load_pages_hash(&mut self) {
        /// Loads the entry:
        /// state ---> [page1_hash || page2_hash || .... || pageN_hash]
        ///
        /// Then, populates `self.pages`. Each page is initialized with `PageEntry::NotModified(page_hash, None)`
        let state = self.state.as_slice();
        let v = self.kv.borrow().get(state);

        assert!(v.is_some(), "Didn't find state: {:?}", state);

        let v = v.unwrap();

        assert!(v.len() % State::len() == 0);

        for (i, ph) in v.chunks_exact(State::len()).enumerate() {
            let ph = PageHash::from(ph);
            trace!("page #{}, has page-hash {:?}", i, ph);

            self.pages[i] = PageEntry::NotModified(ph);
        }
    }

    /// Derives page hash, from its raw `data`.
    #[must_use]
    #[inline]
    pub fn compute_page_hash(&self, page_data: &[u8]) -> PageHash {
        PH::hash(page_data)
    }

    /// Derives page hash for page indexed `page_idx` containing only zeros.
    #[must_use]
    #[inline]
    pub fn compute_zero_page_hash(&self) -> PageHash {
        let zeros_page = crate::page::zero_page();
        self.compute_page_hash(zeros_page.as_ref())
    }

    /// The number of dirty pages
    pub fn dirty_page_count(&self) -> usize {
        self.pages.iter().fold(0, |acc, page| match page {
            PageEntry::NotModified(..) => acc,
            PageEntry::Modified(..) => acc + 1,
            PageEntry::Uninitialized => unreachable!(),
        })
    }

    fn prepare_changeset(&mut self) -> Changeset {
        let mut changes = Vec::new();
        let mut pages_hash = Vec::new();

        for (i, page) in self.pages.drain(..).enumerate() {
            let change = match page {
                PageEntry::NotModified(ph) => pages_hash.push(ph),
                PageEntry::Modified(new_hash, new_data) => {
                    let idx = PageIndex(i as u16);

                    let change = PageChange {
                        idx,
                        new_hash: new_hash.clone(),
                        new_data,
                    };
                    changes.push(change);

                    pages_hash.push(new_hash);
                }
                PageEntry::Uninitialized => unreachable!(),
            };
        }

        let jph = JoinedPagesHash::new(pages_hash);
        let state = SH::hash(&jph);

        Changeset {
            state,
            changes,
            jph,
        }
    }
}

impl<KV, PH, SH> StateAwarePagesStorage for AppPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    #[must_use]
    #[inline]
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

impl<KV, PH, SH> PagesStorage for AppPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    #[must_use]
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        let idx = page_idx.0 as usize;

        match self.pages[idx] {
            PageEntry::NotModified(ph) => {
                let key = &ph.0;
                self.kv.borrow().get(key)
            }
            PageEntry::Modified(..) => panic!("Not allowed to read a dirty page"),
            PageEntry::Uninitialized => unreachable!(),
        }
    }

    fn write_page(&mut self, page_idx: PageIndex, page_data: &[u8]) {
        let idx = page_idx.0 as usize;
        let ph = self.compute_page_hash(page_data);

        self.pages[idx] = PageEntry::Modified(ph, page_data.to_vec());
    }

    fn clear(&mut self) {
        debug!("Clearing pages-storage...");

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
        // Now, we'll compute the new state of the App pages.
        //
        // ```
        // new_state = HASH(page1_hash || page2_hash || ... || pageN_hash)
        // ```

        debug!("About to commit dirty pages to underlying key-value store.");

        let changeset = self.prepare_changeset();

        let mut entries: Vec<(&[u8], &[u8])> = Vec::with_capacity(1 + changeset.changes.len());

        let state_entry = (changeset.state.as_slice(), changeset.jph.as_slice());
        entries.push(state_entry);

        for change in changeset.changes.iter() {
            let k = change.new_hash.as_ref();
            let v = &change.new_data[..];

            let entry = (k, v);
            entries.push(entry);
        }

        // At last, we store under the flat key-value store (`self.kv`) the following new entries:
        //
        // ```
        // new_state  ---> [page1_hash, page2_hash, ..., pageN_hash]
        // page1_hash ---> page1_content
        // page2_hash ---> page2_content
        // ...
        // ...
        // pageN_hash ---> pageN_content
        // ```

        self.kv.borrow_mut().store(&entries);
        self.state = changeset.state;

        self.clear();
    }
}

impl<KV, PH, SH> Drop for AppPages<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    fn drop(&mut self) {
        debug!("dropping `AppPages`...");
    }
}
