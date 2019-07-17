use crate::page::{PageHash, PageIndex, PagesState};
use crate::traits::{KVStore, PageHasher, PagesStateStorage, PagesStorage};
use svm_common::Address;

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

/// TODO: add docs
#[allow(missing_docs)]
pub struct MerklePageStorage<KV, PH> {
    state: PagesState,
    contract_addr: Address,
    uncommitted: HashMap<PageIndex, Vec<u8>>,
    marker: PhantomData<PH>,
    db: Rc<RefCell<KV>>,
}

impl<KV: KVStore<K = PageHash>, PH: PageHasher> MerklePageStorage<KV, PH> {
    pub fn new(contract_addr: Address, db: Rc<RefCell<KV>>, state: PagesState) -> Self {
        Self {
            state,
            db,
            contract_addr,
            uncommitted: HashMap::new(),
            marker: PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    fn compute_page_hash(&self, page_idx: PageIndex, page_data: &[u8]) -> PageHash {
        // PH::hash(self.contract_addr, page_idx)
        unimplemented!()
    }

    #[cfg(test)]
    pub fn uncommitted_len(&self) -> usize {
        self.uncommitted.len()
    }

    fn merkle_proof(
        &self,
        pages_with_changes: &[(PageIndex, PageHash, &[u8])],
        pages_without_changes: &[(PageIndex, PageHash)],
    ) -> PageHash {
        unimplemented!()
    }
}

impl<KV: KVStore<K = PageHash>, PH: PageHasher> PagesStateStorage for MerklePageStorage<KV, PH> {
    fn set_state(&mut self, state: PagesState) {
        self.state = state;
    }

    fn get_state(&self) -> PagesState {
        self.state
    }

    fn get_page_hash(&self, page_idx: PageIndex) -> PageHash {
        unimplemented!()
    }

    fn apply_changes(
        &mut self,
        pages: Vec<(PageIndex, PageHash, Option<&[u8]>)>,
    ) -> (PageHash, Vec<(PageIndex, PageHash)>) {
        panic!()
    }
}

impl<KV: KVStore<K = PageHash>, PH: PageHasher> PagesStorage for MerklePageStorage<KV, PH> {
    #[must_use]
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        // let page_hash = get_page_state(page_idx);
        // self.db.borrow().get(page_hash)
        None
    }

    fn write_page(&mut self, page_idx: PageIndex, data: &[u8]) {
        self.uncommitted.insert(page_idx, data.to_vec());
    }

    fn clear(&mut self) {
        self.uncommitted.clear();
    }

    fn commit(&mut self) {
        let pages_with_changes: Vec<(PageIndex, PageHash, &[u8])> = self
            .uncommitted
            .iter()
            .map(|(&page_idx, page_data)| {
                let ph = self.compute_page_hash(page_idx, page_data);
                (page_idx, ph, page_data.as_slice())
            })
            .collect();

        let pages_without_changes: Vec<(PageIndex, PageHash)> = Vec::new();

        let new_state: PageHash = self.merkle_proof(
            pages_with_changes.as_slice(),
            pages_without_changes.as_slice(),
        );

        /// For each dirty page we calculate its new page-hash.
        /// A page-hash is calculated as:
        /// ```
        /// HASH(contract_addr || page_idx || HASH(page_content))
        /// ```
        ///
        /// Then, once we have each page-hash (we already have the page-hash for each non-dirty page),
        /// we compute the new state (merkle proof) of the Smart Contract state.
        ///
        /// ```
        /// new_state = HASH(page1_hash || page2_hash || ... || pageN_hash)
        /// ```
        ///
        /// At last, we store under the flat key-value store (`self.db`) the following new entries:
        /// ```
        /// new_state  ---> [page1_hash, page2_hash, ..., pageN_hash]
        /// page1_hash ---> page1_content
        /// page2_hash ---> page2_content
        /// ...
        /// ...
        /// pageN_hash ---> pageN_content
        /// ```
        ///
        /// We save only pages that had modifications (otherwise they stayed with the same hash)
        ///
        let changes: Vec<&[(PageHash, &[u8])]> = Vec::with_capacity(pages_with_changes.len() + 1);

        // self.db.borrow_mut().store(changes.as_slice());
        // self.set_root(new_state);

        self.clear();
    }
}
