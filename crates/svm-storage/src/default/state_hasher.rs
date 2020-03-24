use crate::{
    page::{JoinedPagesHash, PageHash, PAGE_HASH_LEN},
    traits::StateHasher,
};

use svm_common::{DefaultKeyHasher, KeyHasher, State};

pub struct DefaultStateHasher;

impl StateHasher for DefaultStateHasher {
    /// Given a slice of `PageHash`. `StateHash` is derived by:
    ///
    /// HASH(page1_hash || page2_hash || ... || pageN_hash)
    ///
    fn hash(jph: &JoinedPagesHash) -> State {
        let hash = DefaultKeyHasher::hash(jph.as_slice());

        State::from(&hash[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{default::DefaultPageHasher, page::PageIndex, traits::PageHasher};

    use svm_common::{Address, DefaultKeyHasher, KeyHasher};

    #[test]
    fn default_state_hasher_sanity() {
        let page1: Vec<u8> = vec![10, 20, 30];
        let page2: Vec<u8> = vec![40, 50, 60];

        let addr = Address::of("something");

        let page1_hash = DefaultPageHasher::hash(addr.clone(), PageIndex(0), &page1);
        let page2_hash = DefaultPageHasher::hash(addr.clone(), PageIndex(1), &page2);

        let mut joined_pages_hash = Vec::with_capacity(PAGE_HASH_LEN * 2);
        joined_pages_hash.extend_from_slice(&page1_hash.0);
        joined_pages_hash.extend_from_slice(&page2_hash.0);
        let expected = StateHash(DefaultKeyHasher::hash(&joined_pages_hash));

        let pages_hash = vec![page1_hash, page2_hash];
        let actual = DefaultStateHasher::hash(&pages_hash);

        assert_eq!(expected, actual);
    }
}
