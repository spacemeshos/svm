use crate::page::{PageHash, PageIndex};
use crate::traits::{PageHasher, PagesStateStorage, PagesStorage, StateHasher};

use svm_common::{Address, State};
use svm_kv::traits::KVStore;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Debug, Clone)]
enum MerklePage {
    Uninitialized,
    NotModified(PageHash),
    Modified(PageHash, Vec<u8>),
}

/// `MerklePageStorage` is an implemetation of the `PagesStorage` trait that is state aware.
/// `KV` - stands for `KVStore`
/// `PH` - stands for `PageHasher`
/// `SH` - stands for `StateHasher`
pub struct MerklePagesStorage<KV, PH, SH> {
    state: State,
    addr: Address,
    pages: Vec<MerklePage>,
    kv: Arc<RefCell<KV>>,
    pages_count: u32,
    marker: PhantomData<(PH, SH)>,
}

impl<KV, PH, SH> MerklePagesStorage<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    /// Creates a new instance of `MerklePageStorage`
    /// * `addr`        - The running contract account address.
    /// * `kv`          - The underlying kv-store used for retrieving a page raw-data when queried by its page-hash serving as a key.
    /// * `state`       - The current contract-storage state prior execution of the current contract transaction.
    /// * `pages_count` - The number of pages consumed by the contract storage (it's a fixed value per-contract).
    pub fn new(addr: Address, kv: Arc<RefCell<KV>>, state: State, pages_count: u32) -> Self {
        let mut storage = Self {
            state,
            kv,
            pages_count,
            addr,
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
        if self.state == State::empty() {
            // `self.state` is `000...0`. It means that state doesn't exist under the key-value store.
            // This happens when a Smart Contract runs for the first time.
            // We initialize each page with its zero-page hash `HASH(addr || page_idx || 0...0)`

            for page_idx in 0..(self.pages_count as usize) {
                let ph = self.compute_zero_page_hash(PageIndex(page_idx as u32));
                self.pages[page_idx] = MerklePage::NotModified(ph);
            }
        } else if let Some(v) = self.kv.borrow().get(self.state.as_slice()) {
            // `v` should be a concatenation of pages-hash. Each page hash consumes exactly 32 bytes.
            assert!(v.len() % 32 == 0);

            for (page_idx, raw_ph) in v.chunks_exact(32).enumerate() {
                let ph = PageHash::from(raw_ph);
                self.pages[page_idx] = MerklePage::NotModified(ph);
            }
        } else {
            panic!("Didn't find state: {:?}", self.state.as_slice());
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn compute_page_hash(&self, page_idx: PageIndex, page_data: &[u8]) -> PageHash {
        PH::hash(self.addr, page_idx, page_data)
    }

    #[must_use]
    #[inline(always)]
    pub fn compute_zero_page_hash(&self, page_idx: PageIndex) -> PageHash {
        let zeros_page = crate::page::zero_page();
        self.compute_page_hash(page_idx, zeros_page.as_ref())
    }

    #[cfg(test)]
    pub fn modified_pages_count(&self) -> usize {
        self.pages.iter().fold(0, |acc, page| match page {
            MerklePage::NotModified(..) => acc,
            MerklePage::Modified(..) => acc + 1,
            MerklePage::Uninitialized => unreachable!(),
        })
    }

    fn prepare_changeset(&self) -> (State, Vec<PageHash>, Vec<(&[u8], &[u8])>) {
        let mut changes = Vec::new();

        let mut pages_hash: Vec<PageHash> = Vec::new();

        for page in self.pages.iter() {
            match page {
                MerklePage::NotModified(ph) => pages_hash.push(*ph),
                MerklePage::Modified(ph, data) => {
                    let change: (&[u8], &[u8]) = (&ph.0, data);
                    changes.push(change);

                    pages_hash.push(*ph);
                }
                MerklePage::Uninitialized => unreachable!(),
            }
        }

        let new_state_hash = SH::hash(pages_hash.as_slice());
        let new_state = State::from(new_state_hash.as_ref());

        (new_state, pages_hash, changes)
    }
}

impl<KV, PH, SH> PagesStateStorage for MerklePagesStorage<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    #[must_use]
    #[inline(always)]
    fn get_state(&self) -> State {
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

impl<KV, PH, SH> PagesStorage for MerklePagesStorage<KV, PH, SH>
where
    KV: KVStore,
    PH: PageHasher,
    SH: StateHasher,
{
    #[must_use]
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        match self.pages[page_idx.0 as usize] {
            MerklePage::NotModified(ph) => self.kv.borrow().get(&ph.0),
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

#[cfg(test)]
mod tests {
    use super::*;

    use svm_common::{Address, DefaultKeyHasher, State};

    use crate::default::DefaultPageHasher;
    use crate::page::zero_page;
    use svm_kv::memory::MemKVStore;

    use std::cell::RefCell;
    use std::rc::Rc;

    macro_rules! join_pages_hash {
        ($pages_hash: expr) => {{
            let mut joined = Vec::new();

            for ph in $pages_hash {
                joined.extend_from_slice(ph.as_ref());
            }

            joined
        }};
    }

    macro_rules! kv_keys_vec {
        ($kv: ident) => {{
            let keys: Vec<Vec<u8>> = $kv.borrow().keys().map(|key| key.clone()).collect();
            keys
        }};
    }

    macro_rules! assert_same_keys {
        ($expected: expr, $actual: expr) => {{
            let mut expected = $expected
                .iter()
                .map(|k| k.to_vec())
                .collect::<Vec<Vec<u8>>>();
            let mut actual = $actual.to_vec();

            expected.sort();
            actual.sort();

            assert_eq!(&expected[..], &actual[..]);
        }};
    }

    macro_rules! assert_key_value {
        ($kv: expr, $key: expr, $expected: expr) => {{
            let actual = $kv.borrow().get(&$key).unwrap();
            assert_eq!($expected, &actual[..]);
        }};
    }

    macro_rules! assert_no_key {
        ($kv: expr, $key: expr) => {{
            assert!($kv.borrow().get(&$key).is_none());
        }};
    }

    macro_rules! assert_dirty_pages_count {
        ($storage: ident, $expected: expr) => {{
            assert_eq!($expected, $storage.modified_pages_count());
        }};
    }

    macro_rules! mem_merkle_pages_setup {
        ($addr_expr: expr, $addr: ident, $storage: ident, $kv: ident, $pages_count: expr) => {
            let $addr = Address::from($addr_expr as u32);
            let $kv = Arc::new(RefCell::new(MemKVStore::new()));

            #[allow(unused_mut)]
            let mut $storage =
                $crate::memory::MemMerklePages::new($addr, Arc::clone(&$kv), State::empty(), $pages_count);
        };
    }

    macro_rules! mem_merkle_pages_open {
        ($addr_expr: expr, $addr: ident, $storage: ident, $kv: ident, $state: expr, $pages_count: expr) => {
            let $addr = Address::from($addr_expr as u32);

            #[allow(unused_mut)]
            let mut $storage = $crate::memory::MemMerklePages::new($addr, Arc::clone(&$kv), $state, $pages_count);
        };
    }

    macro_rules! assert_state {
        ($expected: expr, $storage: ident) => {{
            assert_eq!($expected, $storage.get_state());
        }};
    }

    macro_rules! assert_page {
        ($storage: ident, $page_idx: expr, $expected: expr) => {{
            assert_eq!($expected, $storage.read_page(PageIndex($page_idx)));
        }};
    }

    macro_rules! compute_page_hash {
        ($addr: ident, $page_idx: expr, $data: expr) => {{
            use $crate::default::DefaultPageHasher;;

            DefaultPageHasher::hash($addr, PageIndex($page_idx), $data)
        }};
    }

    macro_rules! compute_state {
        ($jph: expr) => {{
            use svm_common::DefaultKeyHasher;
            use svm_common::KeyHasher;

            let state = Some($jph.as_slice())
                .map(|ref_jph| {
                    let h = DefaultKeyHasher::hash(ref_jph);
                    State::from(h.as_ref())
                })
                .unwrap();

            state
        }};
    }

    #[test]
    fn first_run_with_no_modifications_no_commit() {
        mem_merkle_pages_setup!(0x11_22_33_44, addr, storage, kv, 3);

        assert_dirty_pages_count!(storage, 0);
        assert_state!(State::empty(), storage);
        assert_page!(storage, 0, None);
    }

    #[test]
    fn first_run_with_no_modifications_with_commit() {
        mem_merkle_pages_setup!(0x11_22_33_44, addr, storage, kv, 3);
        assert_dirty_pages_count!(storage, 0);
        storage.commit();

        let ph0 = compute_page_hash!(addr, 0, &zero_page());
        let ph1 = compute_page_hash!(addr, 1, &zero_page());
        let ph2 = compute_page_hash!(addr, 2, &zero_page());

        let jph = join_pages_hash!(&[ph0, ph1, ph2]);
        let state = compute_state!(jph);

        assert_state!(state, storage);
        assert_same_keys!(vec![state.bytes()], kv_keys_vec!(kv));

        assert_no_key!(&kv, ph0.0);
        assert_no_key!(&kv, ph1.0);
        assert_no_key!(&kv, ph2.0);
        assert_page!(storage, 0, None);
        assert_page!(storage, 1, None);
        assert_page!(storage, 2, None);
        assert_dirty_pages_count!(storage, 0);
    }

    #[test]
    fn first_run_with_one_modified_page() {
        mem_merkle_pages_setup!(0x11_22_33_44, addr, storage, kv, 3);

        storage.write_page(PageIndex(0), &[10, 20, 30]);
        assert_dirty_pages_count!(storage, 1);
        storage.commit();

        let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
        let ph1 = compute_page_hash!(addr, 1, &zero_page());
        let ph2 = compute_page_hash!(addr, 2, &zero_page());
        let jph = join_pages_hash!(&[ph0, ph1, ph2]);
        let state = compute_state!(jph);

        assert_state!(state, storage);
        assert_same_keys!(vec![state.bytes(), ph0.0], kv_keys_vec!(kv));
        assert_key_value!(kv, state.bytes(), jph);
        assert_key_value!(kv, ph0.0, [10, 20, 30]);
        assert_page!(storage, 0, Some(vec![10, 20, 30]));
        assert_page!(storage, 1, None);
        assert_dirty_pages_count!(storage, 0);
    }

    #[test]
    fn first_run_with_two_modified_pages() {
        mem_merkle_pages_setup!(0x11_22_33_44, addr, storage, kv, 2);

        storage.write_page(PageIndex(0), &[10, 20, 30]);
        storage.write_page(PageIndex(1), &[40, 50, 60]);
        assert_dirty_pages_count!(storage, 2);
        storage.commit();

        let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
        let ph1 = compute_page_hash!(addr, 1, &[40, 50, 60]);
        let jph = join_pages_hash!(&[ph0, ph1]);
        let state = compute_state!(jph);

        assert_state!(state, storage);
        assert_same_keys!(vec![state.bytes(), ph0.0, ph1.0], kv_keys_vec!(kv));
        assert_key_value!(kv, state.bytes(), jph);
        assert_key_value!(kv, ph0.0, [10, 20, 30]);
        assert_key_value!(kv, ph1.0, [40, 50, 60]);
        assert_page!(storage, 0, Some(vec![10, 20, 30]));
        assert_page!(storage, 1, Some(vec![40, 50, 60]));
        assert_dirty_pages_count!(storage, 0);
    }

    #[test]
    fn second_run_after_first_run_with_no_modifications() {
        // 1st run
        mem_merkle_pages_setup!(0x11_22_33_44, addr, storage, kv, 3);
        storage.commit();
        let old_state = storage.get_state();

        // 2nd run
        mem_merkle_pages_open!(0x11_22_33_44, addr, storage, kv, old_state, 3);
        storage.write_page(PageIndex(0), &[10, 20, 30]);
        storage.write_page(PageIndex(1), &[40, 50, 60]);
        storage.commit();

        // modifying pages `0` and `1`
        let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
        let ph1 = compute_page_hash!(addr, 1, &[40, 50, 60]);
        let ph2 = compute_page_hash!(addr, 2, &zero_page());
        let jph = join_pages_hash!(&[ph0, ph1, ph2]);
        let new_state = compute_state!(jph);

        assert_same_keys!(
            vec![old_state.bytes(), new_state.bytes(), ph0.0, ph1.0],
            kv_keys_vec!(kv)
        );

        assert_key_value!(kv, new_state.bytes(), jph);
        assert_key_value!(kv, ph0.0, [10, 20, 30]);
        assert_key_value!(kv, ph1.0, [40, 50, 60]);
        assert_no_key!(kv, ph2.0);
    }

    #[test]
    fn second_run_after_first_run_with_modifications() {
        // 1st run
        mem_merkle_pages_setup!(0x11_22_33_44, addr, storage, kv, 3);
        storage.write_page(PageIndex(0), &[11, 22, 33]);
        storage.commit();
        let old_state = storage.get_state();

        // 2nd run
        mem_merkle_pages_open!(0x11_22_33_44, addr, storage, kv, old_state, 3);
        storage.write_page(PageIndex(0), &[10, 20, 30]);
        storage.write_page(PageIndex(1), &[40, 50, 60]);
        storage.commit();

        // modifying pages `0` and `1`
        let ph0_old = compute_page_hash!(addr, 0, &[11, 22, 33]);
        let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
        let ph1 = compute_page_hash!(addr, 1, &[40, 50, 60]);
        let ph2 = compute_page_hash!(addr, 2, &zero_page());
        let jph = join_pages_hash!(&[ph0, ph1, ph2]);
        let new_state = compute_state!(jph);

        assert_same_keys!(
            vec![old_state.bytes(), new_state.bytes(), ph0_old.0, ph0.0, ph1.0],
            kv_keys_vec!(kv)
        );

        assert_key_value!(kv, new_state.bytes(), jph);
        assert_key_value!(kv, ph0.0, [10, 20, 30]);
        assert_key_value!(kv, ph1.0, [40, 50, 60]);
        assert_no_key!(kv, ph2.0);
    }

    #[test]
    fn third_run_rollbacks_to_after_first_run() {
        // 1st run
        mem_merkle_pages_setup!(0x11_22_33_44, addr, storage, kv, 3);
        storage.write_page(PageIndex(0), &[11, 22, 33]);
        storage.commit();
        let state_1 = storage.get_state();

        // 2nd run
        mem_merkle_pages_open!(0x11_22_33_44, addr, storage, kv, state_1, 3);
        storage.write_page(PageIndex(0), &[10, 20, 30]);
        storage.write_page(PageIndex(1), &[40, 50, 60]);
        storage.commit();
        let state_2 = storage.get_state();

        // 3rd run (rollbacks to `state_1` initial state)
        mem_merkle_pages_open!(0x11_22_33_44, addr, storage, kv, state_1, 3);

        let ph0_1 = compute_page_hash!(addr, 0, &[11, 22, 33]);
        let ph1_1 = compute_page_hash!(addr, 1, &zero_page());
        let ph2_1 = compute_page_hash!(addr, 2, &zero_page());

        let ph0_2 = compute_page_hash!(addr, 0, &[10, 20, 30]);
        let ph1_2 = compute_page_hash!(addr, 1, &[40, 50, 60]);
        let ph2_2 = compute_page_hash!(addr, 2, &zero_page());
        let jph = join_pages_hash!(&[ph0_1, ph1_1, ph2_1]);

        assert_same_keys!(
            vec![state_1.bytes(), state_2.bytes(), ph0_1.0, ph0_2.0, ph1_2.0],
            kv_keys_vec!(kv)
        );

        assert_state!(state_1, storage);
        assert_key_value!(kv, state_1.bytes(), jph);

        // 4th run (rollbacks to `state_2` initial state)
        mem_merkle_pages_open!(0x11_22_33_44, addr, storage, kv, state_2, 3);
        let jph = join_pages_hash!(&[ph0_2, ph1_2, ph2_2]);

        assert_same_keys!(
            vec![state_1.bytes(), state_2.bytes(), ph0_1.0, ph0_2.0, ph1_2.0],
            kv_keys_vec!(kv)
        );

        assert_key_value!(kv, state_2.bytes(), jph);
        assert_state!(state_2, storage);
    }
}
