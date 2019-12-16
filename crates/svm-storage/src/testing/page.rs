use svm_common::{Address, DefaultKeyHasher, KeyHasher, State};

use crate::default::{DefaultPageHasher, DefaultPageIndexHasher};
use crate::page::{PageHash, PageIndex};
use crate::traits::{PageHasher, PageIndexHasher};

/// An helper for computing a page default hash using `DefaultPageIndexHasher`
pub fn default_page_hash(addr: &Address, page_idx: u32, data: &[u8]) -> PageHash {
    DefaultPageHasher::hash(addr.clone(), PageIndex(page_idx), data)
}

/// An helper for computing page-index hashes using `DefaultPageIndexHasher`
pub fn default_page_index_hash(addr: u32, page_idx: u32) -> [u8; 32] {
    let addr = Address::from(addr as u32);

    DefaultPageIndexHasher::hash(addr, PageIndex(page_idx))
}

pub fn fill_page(page: &mut [u8], items: &[(usize, u8)]) {
    for (i, b) in items {
        page[*i] = *b;
    }
}

pub fn concat_pages_hash(pages_hash: &[PageHash]) -> Vec<u8> {
    let mut res = Vec::new();

    for ph in pages_hash.iter() {
        res.extend_from_slice(ph.as_ref());
    }

    res
}

pub fn compute_pages_state(pages_hash: &[PageHash]) -> State {
    let concat_ph = concat_pages_hash(pages_hash);

    let state = Some(concat_ph.as_slice())
        .map(|jph| {
            let h = DefaultKeyHasher::hash(jph);
            State::from(h.as_ref())
        })
        .unwrap();

    state
}
