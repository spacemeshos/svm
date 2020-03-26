use super::{PageHash, PageIndex};

/// Concatenates a vector of `PageHash`.
/// Since we need to use thee result in a few places, we have `JoinedPagesHash` as a cache.
pub struct JoinedPagesHash {
    bytes: Vec<u8>,
    pages_hash: Vec<PageHash>,
}

impl JoinedPagesHash {
    /// Creates a new instnace
    pub fn new(pages_hash: Vec<PageHash>) -> Self {
        let bytes = pages_hash.iter().flat_map(|ph| ph.0.to_vec()).collect();

        Self { pages_hash, bytes }
    }

    /// Returns the concatenated pages-hash as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..]
    }

    /// Returns the `PageHash` of page `page_idx`.
    pub fn get_page_hash(&self, page_idx: PageIndex) -> &PageHash {
        let idx = page_idx.0 as usize;

        &self.pages_hash[idx]
    }
}
