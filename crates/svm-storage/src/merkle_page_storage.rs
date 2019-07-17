use crate::page::{PageHash, PageIndex};
use crate::traits::{KVStore, PagesState, PagesStorage};
use svm_common::Address;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// TODO: add docs
#[allow(missing_docs)]
pub struct MerklePageStorage<KV> {
    root: [u8; 32],
    contract_addr: Address,
    uncommitted: HashMap<PageIndex, Vec<u8>>,
    db: Rc<RefCell<KV>>,
}

impl<KV: KVStore<K = PageHash>> MerklePageStorage<KV> {
    pub fn new(contract_addr: Address, db: Rc<RefCell<KV>>, root: [u8; 32]) -> Self {
        Self {
            root,
            db,
            contract_addr,
            uncommitted: HashMap::new(),
        }
    }

    pub fn set_root(&mut self, root: [u8; 32]) {
        self.root = root;
    }

    pub fn get_root(&self) -> [u8; 32] {
        self.root
    }

    #[must_use]
    #[inline(always)]
    fn compute_page_hash(&self, page_idx: PageIndex) -> PageHash {
        unimplemented!()
    }

    #[cfg(test)]
    pub fn uncommitted_len(&self) -> usize {
        self.uncommitted.len()
    }
}

impl<KV: KVStore<K = [u8; 32]>> PagesState for MerklePageStorage<KV> {
    fn get_pages(&self, state: &[u8]) -> Vec<(PageIndex, PageHash)> {
        Vec::new()
    }

    fn compute_state(pages: Vec<(PageIndex, PageHash, Option<&[u8]>)>) -> Vec<u8> {
        Vec::new()
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
        let changes: Vec<(PageHash, &[u8])> = self
            .uncommitted
            .iter()
            .map(|(page_idx, data)| {
                let ph = self.compute_page_hash(*page_idx);
                (ph, data.as_slice())
            })
            .collect();

        // self.db.borrow_mut().store(changes.as_slice());
        //
        // self.clear();
    }

    fn commit(&mut self) {
        // let old_pages = self.get_pages(...)
        // let new_pages = merge (old_pages, dirty pages)
        //
        // new_root = self.compute_state(new_pages);
        // set_root(new_root);
    }
}
