use super::{PageHash, PageIndex};

pub struct JoinedPagesHash {
    bytes: Vec<u8>,
    pages_hash: Vec<PageHash>,
}

impl JoinedPagesHash {
    pub fn new(pages_hash: Vec<PageHash>) -> Self {
        let bytes = pages_hash.iter().flat_map(|ph| ph.0.to_vec()).collect();

        Self { pages_hash, bytes }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn get_page_hash(&self, page_idx: PageIndex) -> &PageHash {
        let idx = page_idx.0 as usize;

        &self.pages_hash[idx]
    }
}
