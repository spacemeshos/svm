use crate::{page::JoinedPagesHash, traits::StateHasher};

use svm_common::{DefaultKeyHasher, KeyHasher, State};

/// Default `StateHasher` implementation.
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

    use crate::{default::DefaultPageHasher, page::JoinedPagesHash, traits::PageHasher};

    use svm_common::{DefaultKeyHasher, KeyHasher};

    #[test]
    fn default_state_hasher_sanity() {
        let page1 = vec![10, 20, 30];
        let page2 = vec![40, 50, 60];

        let hash1 = DefaultPageHasher::hash(&page1);
        let hash2 = DefaultPageHasher::hash(&page2);

        let jph = JoinedPagesHash::new(vec![hash1.clone(), hash2.clone()]);
        let bytes = DefaultKeyHasher::hash(jph.as_slice());
        let expected = State::from(&bytes[..]);

        let actual = DefaultStateHasher::hash(&jph);

        assert_eq!(expected, actual);
    }
}
