use crate::{page::PageHash, traits::PageHasher};

use svm_common::{DefaultKeyHasher, KeyHasher};

/// Default `PageHasher` implementation.
pub struct DefaultPageHasher;

impl PageHasher for DefaultPageHasher {
    /// Hashes the page content.
    fn hash(data: &[u8]) -> PageHash {
        let hash = DefaultKeyHasher::hash(data);

        PageHash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::page::PageIndex;

    #[test]
    fn default_page_hasher_sanity() {
        let page_data = vec![10, 20, 30];
        let hash = DefaultKeyHasher::hash(&page_data);
        let expected = PageHash(hash);

        let actual = DefaultPageHasher::hash(&page_data);

        assert_eq!(expected, actual);
    }
}
