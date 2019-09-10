use crate::page::{PageHash, PAGE_HASH_LEN};
use crate::state::StateHash;
use crate::traits::StateHasher;
use svm_common::{DefaultKeyHasher, KeyHasher};

use std::marker::PhantomData;

pub struct StateHasherImpl<SH> {
    marker: PhantomData<SH>,
}

impl<KH> StateHasher for StateHasherImpl<KH>
where
    KH: KeyHasher<Hash = [u8; 32]>,
{
    /// Given a slice of `PageHash`. `StateHash` is derived by:
    ///
    /// HASH(page1_hash || page2_hash || ... || pageN_hash)
    ///
    fn hash(pages_hash: &[PageHash]) -> StateHash {
        let mut joined_pages_hash: Vec<u8> =
            Vec::with_capacity(pages_hash.len() as usize * PAGE_HASH_LEN);

        for ph in pages_hash {
            joined_pages_hash.extend_from_slice(&ph.0);
        }

        let hash = KH::hash(&joined_pages_hash);

        StateHash(hash)
    }
}

/// A default implementation for `StateHasher` trait.
pub type DefaultStateHasher = StateHasherImpl<DefaultKeyHasher>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default::DefaultPageHasher;
    use crate::page::PageIndex;
    use crate::traits::PageHasher;
    use svm_common::{Address, DefaultKeyHasher, KeyHasher};

    #[test]
    fn default_state_hasher_sanity() {
        let page1: Vec<u8> = vec![10, 20, 30];
        let page2: Vec<u8> = vec![40, 50, 60];

        let addr = Address::from(0xAABBCC);

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
