use crate::page::{PageHash, PageIndex, PagesState};
use crate::traits::{KVStore, PageHasher, PagesStateStorage, PagesStorage};
use svm_common::{Address, KeyHasher};

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KVStoreKey([u8; 32]);

impl AsRef<[u8]> for KVStoreKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
enum MerklePage {
    Uninitialized,
    NotModified(PageHash),
    Modified(PageHash, Vec<u8>),
}

pub struct MerklePageStorage<KV, KH, PH> {
    state: PagesState,
    contract_addr: Address,
    pages: Vec<MerklePage>,
    kv: Rc<RefCell<KV>>,
    pages_count: u32,
    marker: PhantomData<(PH, KH)>,
}

impl<KV, KH, PH> MerklePageStorage<KV, KH, PH>
where
    KV: KVStore<K = KVStoreKey>,
    KH: KeyHasher,
    PH: PageHasher,
{
    pub fn new(
        contract_addr: Address,
        kv: Rc<RefCell<KV>>,
        state: PagesState,
        pages_count: u32,
    ) -> Self {
        let mut storage = Self {
            state,
            kv,
            pages_count,
            contract_addr,
            pages: vec![MerklePage::Uninitialized; pages_count as usize],
            marker: PhantomData,
        };

        storage.init_pages_state();

        storage
    }

    /// Loads the entry:
    /// state ---> [page1_hash || page2_hash || .... || pageN_hash]
    ///
    /// Then, populates `self.pages`. Each page is initialized with `MerklePage::NotModified(page_hash)`
    fn init_pages_state(&mut self) {
        let state_key = KVStoreKey(self.state.0);

        if self.state == PagesState::empty() {
            // `self.tate` is `000...0`. It means that state doesn't exist under the key-value store.
            // This happens when a Smart Contract runs for the first time.
            // We initialize each page with its zero-page hash `HASH(contract_addr || page_idx || 0...0)`

            for page_idx in 0..(self.pages_count as usize) {
                let ph = self.compute_zero_page_hash(PageIndex(page_idx as u32));
                self.pages[page_idx] = MerklePage::Modified(ph, Vec::new());
            }
        } else if let Some(v) = self.kv.borrow().get(state_key) {
            // `v` should be a concatenation of pages-hash. Each page hash consumes exactly 32 bytes
            assert!(v.len() % 32 == 0);

            for (page_idx, raw_ph) in v.chunks_exact(32).enumerate() {
                let ph = PageHash::from(raw_ph);
                self.pages[page_idx] = MerklePage::NotModified(ph);
            }
        } else {
            panic!("Didn't find state: {:?}", self.state.0);
        }
    }

    #[must_use]
    #[inline(always)]
    fn compute_page_hash(&self, page_idx: PageIndex, page_data: &[u8]) -> PageHash {
        PH::hash(self.contract_addr, page_idx, page_data)
    }

    #[must_use]
    #[inline(always)]
    fn compute_zero_page_hash(&self, page_idx: PageIndex) -> PageHash {
        PH::hash(self.contract_addr, page_idx, [0; 32].as_ref())
    }

    #[cfg(test)]
    pub fn modified_pages_count(&self) -> usize {
        self.pages.iter().fold(0, |acc, page| match page {
            MerklePage::NotModified(..) => acc,
            MerklePage::Modified(..) => acc + 1,
            MerklePage::Uninitialized => unreachable!(),
        })
    }

    fn prepare_changeset(&self) -> (PagesState, Vec<u8>, Vec<(KVStoreKey, &[u8])>) {
        let mut changes = Vec::new();

        let mut joined_pages_hash: Vec<u8> = Vec::with_capacity(self.pages_count as usize * 32);

        // `joined_pages_hash = page1_hash || page2_hash || ... || pageN_hash`

        for (page_idx, page) in self.pages.iter().enumerate() {
            match page {
                MerklePage::NotModified(ph) => joined_pages_hash.extend_from_slice(ph.as_ref()),
                MerklePage::Modified(ph, data) => {
                    let key = KVStoreKey(ph.0);
                    let change = (key, data.as_slice());

                    changes.push(change);

                    joined_pages_hash.extend_from_slice(ph.as_ref());
                }
                MerklePage::Uninitialized => unreachable!(),
            }
        }

        let new_state_hash = KH::hash(&joined_pages_hash);
        let new_state = PagesState::from(new_state_hash.as_ref());

        (new_state, joined_pages_hash, changes)
    }
}

impl<KV, KH, PH> PagesStateStorage for MerklePageStorage<KV, KH, PH>
where
    KV: KVStore<K = KVStoreKey>,
    KH: KeyHasher,
    PH: PageHasher,
{
    #[inline(always)]
    fn set_state(&mut self, state: PagesState) {
        self.state = state;
    }

    #[must_use]
    #[inline(always)]
    fn get_state(&self) -> PagesState {
        self.state
    }

    #[must_use]
    fn get_page_hash(&self, page_idx: PageIndex) -> PageHash {
        match self.pages[page_idx.0 as usize] {
            MerklePage::NotModified(ph) => ph,
            MerklePage::Modified(ph, _) => ph,
            MerklePage::Uninitialized => unreachable!(),
        }
    }
}

impl<KV, KH, PH> PagesStorage for MerklePageStorage<KV, KH, PH>
where
    KV: KVStore<K = KVStoreKey>,
    KH: KeyHasher,
    PH: PageHasher,
{
    #[must_use]
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        match self.pages[page_idx.0 as usize] {
            MerklePage::NotModified(ph) => None,
            MerklePage::Modified(..) => panic!("Not allowed to read a dirty page"),
            MerklePage::Uninitialized => unreachable!(),
        }
    }

    fn write_page(&mut self, page_idx: PageIndex, page_data: &[u8]) {
        let ph = self.compute_page_hash(page_idx, page_data);

        self.pages[page_idx.0 as usize] = MerklePage::Modified(ph, page_data.to_vec());
    }

    fn clear(&mut self) {
        for page in &mut self.pages {
            match page {
                MerklePage::Modified(ph, ..) => *page = MerklePage::NotModified(*ph),
                MerklePage::NotModified(..) => (),
                MerklePage::Uninitialized => unreachable!(),
            }
        }
    }

    fn commit(&mut self) {
        // We have each page-hash (dirty and non-dirty) under `self.pages`
        // Now, we'll compute the new state (merkle proof) of the Smart Contract.
        //
        // ```
        // new_state = HASH(page1_hash || page2_hash || ... || pageN_hash)
        // ```

        let (new_state, joined_pages_hash, changeset) = self.prepare_changeset();

        let mut entries: Vec<(KVStoreKey, &[u8])> = Vec::with_capacity(1 + changeset.len());

        entries.push((KVStoreKey(new_state.0), joined_pages_hash.as_slice()));

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
        self.set_state(new_state);

        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svm_common::{Address, DefaultKeyHasher};

    use crate::default::DefaultPageHasher;
    use crate::memory::MemKVStore;
    use crate::page::PagesState;
    use crate::traits::KVStore;

    use std::cell::RefCell;
    use std::rc::Rc;

    type MemMerklePageStorage<K> =
        MerklePageStorage<MemKVStore<K>, DefaultKeyHasher, DefaultPageHasher>;

    fn compute_page_hash(addr: Address, page_idx: PageIndex, data: &[u8]) -> PageHash {
        DefaultPageHasher::hash(addr, page_idx, data)
    }

    fn join_pages_hash(pages_hash: &[PageHash]) -> Vec<u8> {
        let mut joined = Vec::new();

        for ph in pages_hash {
            joined.extend_from_slice(ph.as_ref());
        }

        joined
    }

    fn assert_same_keys(expected: &[KVStoreKey], actual: &[KVStoreKey]) {
        let mut expected = expected.to_vec();
        let mut actual = actual.to_vec();

        expected.sort();
        actual.sort();

        assert_eq!(expected, actual);
    }

    fn assert_key_value(db: &Rc<RefCell<MemKVStore<KVStoreKey>>>, key: [u8; 32], expected: &[u8]) {
        let actual = db.borrow().get(KVStoreKey(key)).unwrap();

        assert_eq!(expected, &actual[..]);
    }

    #[test]
    fn first_run_with_no_modifications() {
        let addr = Address::from(0x11_22_33_44 as u32);
        let kv = Rc::new(RefCell::new(MemKVStore::new()));
        let kv_clone = Rc::clone(&kv);
        let pages_count = 3;

        let mut storage =
            MemMerklePageStorage::new(addr, kv_clone, PagesState::empty(), pages_count);
        storage.commit();

        let ph0 = compute_page_hash(addr, PageIndex(0), &[0; 32]);
        let ph1 = compute_page_hash(addr, PageIndex(1), &[0; 32]);
        let ph2 = compute_page_hash(addr, PageIndex(2), &[0; 32]);

        let jph: Vec<u8> = join_pages_hash(&[ph0, ph1, ph2]);

        let state = Some(jph.as_slice())
            .map(|ref_jph| {
                let h = DefaultKeyHasher::hash(ref_jph);
                PagesState::from(h.as_ref())
            })
            .unwrap();

        let expected_keys: Vec<KVStoreKey> = vec![state.0, ph0.0, ph1.0, ph2.0]
            .iter()
            .map(|v| KVStoreKey(*v))
            .collect();

        let actual_keys: Vec<KVStoreKey> = kv.borrow().keys().map(|key| *key).collect();

        assert_same_keys(&expected_keys[..], &actual_keys[..]);
        assert_key_value(&kv, state.0, &jph[..]);
        assert_key_value(&kv, ph0.0, &[]);
        assert_key_value(&kv, ph1.0, &[]);
        assert_eq!(state, storage.get_state());
    }

    #[test]
    fn first_run_with_one_modified_page() {
        let addr = Address::from(0x11_22_33_44 as u32);
        let kv = Rc::new(RefCell::new(MemKVStore::new()));
        let kv_clone = Rc::clone(&kv);
        let pages_count = 2;

        let mut storage =
            MemMerklePageStorage::new(addr, kv_clone, PagesState::empty(), pages_count);
        storage.write_page(PageIndex(0), &[10, 20, 30]);
        storage.commit();

        let ph0 = compute_page_hash(addr, PageIndex(0), &[10, 20, 30]);
        let ph1 = compute_page_hash(addr, PageIndex(1), &[0; 32]);
        let jph = join_pages_hash(&[ph0, ph1]);

        let state = Some(jph.as_slice())
            .map(|ref_jph| {
                let state = DefaultKeyHasher::hash(ref_jph);
                PagesState::from(state.as_ref())
            })
            .unwrap();

        let expected_keys: Vec<KVStoreKey> = vec![state.0, ph0.0, ph1.0]
            .iter()
            .map(|v| KVStoreKey(*v))
            .collect();

        let actual_keys: Vec<KVStoreKey> = kv.borrow().keys().map(|key| *key).collect();

        assert_same_keys(&expected_keys[..], &actual_keys[..]);
        assert_key_value(&kv, state.0, &jph[..]);
        assert_key_value(&kv, ph0.0, &[10, 20, 30]);
        assert_key_value(&kv, ph1.0, &[]);
        assert_eq!(state, storage.get_state());
    }

    #[test]
    fn first_run_with_two_modified_pages() {
        let addr = Address::from(0x11_22_33_44 as u32);
        let kv = Rc::new(RefCell::new(MemKVStore::new()));
        let kv_clone = Rc::clone(&kv);
        let pages_count = 2;

        let mut storage =
            MemMerklePageStorage::new(addr, kv_clone, PagesState::empty(), pages_count);
        storage.write_page(PageIndex(0), &[10, 20, 30]);
        storage.write_page(PageIndex(1), &[40, 50, 60]);
        storage.commit();

        let ph0 = compute_page_hash(addr, PageIndex(0), &[10, 20, 30]);
        let ph1 = compute_page_hash(addr, PageIndex(1), &[40, 50, 60]);
        let jph = join_pages_hash(&[ph0, ph1]);

        let state = Some(jph.as_slice())
            .map(|ref_jph| {
                let state = DefaultKeyHasher::hash(ref_jph);
                PagesState::from(state.as_ref())
            })
            .unwrap();

        let expected_keys: Vec<KVStoreKey> = vec![state.0, ph0.0, ph1.0]
            .iter()
            .map(|v| KVStoreKey(*v))
            .collect();

        let actual_keys: Vec<KVStoreKey> = kv.borrow().keys().map(|key| *key).collect();

        assert_same_keys(&expected_keys[..], &actual_keys[..]);
        assert_key_value(&kv, state.0, &jph[..]);
        assert_key_value(&kv, ph0.0, &[10, 20, 30]);
        assert_key_value(&kv, ph1.0, &[40, 50, 60]);
        assert_eq!(state, storage.get_state());
    }
}
