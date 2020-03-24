use svm_common::{Address, DefaultKeyHasher, KeyHasher, State};

use crate::{
    default::DefaultPageHasher,
    page::{PageHash, PageIndex},
    traits::PageHasher,
};

/// Default page hash helper
pub fn default_page_hash(data: &[u8]) -> PageHash {
    DefaultPageHasher::hash(data)
}

/// Fills page with input `items` starting from page offset zero.
pub fn fill_page(page: &mut [u8], items: &[(usize, u8)]) {
    for (i, b) in items {
        page[*i] = *b;
    }
}

/// Concatenates pages-hash into one vector of bytes.
pub fn concat_pages_hash(pages_hash: &[PageHash]) -> Vec<u8> {
    let mut res = Vec::new();

    for ph in pages_hash.iter() {
        res.extend_from_slice(ph.as_ref());
    }

    res
}

/// Derives the app new `State` by its pages-hash.
pub fn compute_pages_state(pages_hash: &[PageHash]) -> State {
    let concat_ph = concat_pages_hash(pages_hash);

    Some(concat_ph.as_slice())
        .map(|jph| {
            let h = DefaultKeyHasher::hash(jph);
            State::from(h.as_ref())
        })
        .unwrap()
}
