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

    #[test]
    fn default_page_hasher_sanity() {
        let page_addr = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x44, 0x33, 0x22, 0x14,
        ];

        let page_data_hash = DefaultKeyHasher::hash(&[10, 20, 30]);
        let mut data = Vec::with_capacity(page_addr.len() + page_data_hash.len());
        data.extend_from_slice(&page_addr);
        data.extend_from_slice(&page_data_hash);
        let expected = PageHash(DefaultKeyHasher::hash(data.as_slice()));

        let addr = Address::from(0x44_33_22_11);
        let page_idx = PageIndex(3);
        let page_data = vec![10, 20, 30];

        let actual = DefaultPageHasher::hash(addr, page_idx, page_data.as_slice());

        assert_eq!(expected, actual);
    }
}
