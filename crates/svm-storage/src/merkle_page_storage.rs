use crate::page::{PageHash, PageIndex};
use crate::traits::{KVStore, PagesState, PagesStorage};
use svm_common::Address;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// TODO: add docs
#[allow(missing_docs)]
pub struct MerklePageStorage<KV> {
    root: PageHash,
    contract_addr: Address,
    uncommitted: HashMap<PageIndex, Vec<u8>>,
    db: Rc<RefCell<KV>>,
}

impl<KV: KVStore<K = PageHash>> MerklePageStorage<KV> {
    pub fn new(contract_addr: Address, db: Rc<RefCell<KV>>, root: PageHash) -> Self {
        Self {
            root,
            db,
            contract_addr,
            uncommitted: HashMap::new(),
        }
    }

    pub fn set_root(&mut self, root: PageHash) {
        self.root = root;
    }

    pub fn get_root(&self) -> PageHash {
        self.root
    }

    #[must_use]
    #[inline(always)]
    fn compute_page_hash(&self, page_idx: PageIndex) -> PageHash {
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

impl<KV: KVStore<K = PageHash>> PagesState for MerklePageStorage<KV> {
    fn get_pages_state(&self, state: PageHash) -> Vec<(PageIndex, PageHash)> {
        Vec::new()
    }

    fn compute_pages_state(
        pages: Vec<(PageIndex, PageHash, Option<&[u8]>)>,
    ) -> (PageHash, Vec<(PageIndex, PageHash)>) {
        panic!()
    }
}

impl<KV: KVStore<K = PageHash>> PagesStorage for MerklePageStorage<KV> {
    #[must_use]
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>> {
        let ph = self.compute_page_hash(page_idx);

        self.db.borrow().get(ph)
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
            .map(|(&page_idx, data)| {
                let ph = self.compute_page_hash(page_idx);
                (page_idx, ph, data.as_slice())
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
