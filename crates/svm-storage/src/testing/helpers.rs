use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, DefaultKeyHasher, KeyHasher, State};
use svm_kv::memory::MemKVStore;

use crate::default::{DefaultPageHasher, DefaultPageIndexHasher};
use crate::memory::MemContractPages;
use crate::page::{PageHash, PageIndex};
use crate::traits::{PageHasher, PageIndexHasher};

pub fn compute_pages_state(pages_hash: &[PageHash]) -> State {
    let mut joined_ph = Vec::new();

    for ph in pages_hash.iter() {
        joined_ph.extend_from_slice(ph.as_ref());
    }

    let state = Some(joined_ph.as_slice())
        .map(|jph| {
            let h = DefaultKeyHasher::hash(jph);
            State::from(h.as_ref())
        })
        .unwrap();

    state
}

#[macro_export]
macro_rules! compute_contract_state {
    // `jph` stands for `joined-pages-hash`
    ($jph: expr) => {{
        use svm_common::{DefaultKeyHasher, KeyHasher, State};

        let state = Some($jph.as_slice())
            .map(|ref_jph| {
                let h = DefaultKeyHasher::hash(ref_jph);
                State::from(h.as_ref())
            })
            .unwrap();

        state
    }};
}

pub fn contract_pages_init(
    addr: u32,
    pages_count: u32,
) -> (Address, Rc<RefCell<MemKVStore>>, MemContractPages) {
    let addr = Address::from(addr as u32);
    let kv = Rc::new(RefCell::new(MemKVStore::new()));

    let pages = MemContractPages::new(addr.clone(), Rc::clone(&kv), State::empty(), pages_count);

    (addr, kv, pages)
}

/// An helper for computing a page default hash using `DefaultPageIndexHasher`
pub fn default_page_hash(addr: &Address, page_idx: u32, data: &[u8]) -> PageHash {
    DefaultPageHasher::hash(addr.clone(), PageIndex(page_idx), data)
}

/// An helper for computing page-index hashes using `DefaultPageIndexHasher`
pub fn default_page_index_hash(addr: u32, page_idx: u32) -> [u8; 32] {
    let addr = Address::from(addr as u32);

    DefaultPageIndexHasher::hash(addr, PageIndex(page_idx))
}
