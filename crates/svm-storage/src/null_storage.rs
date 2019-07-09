use super::traits::PagesStorage;
use super::{PageCache, PageSliceCache};

/// A do-nothing `PagesStorage`.
pub struct NullPagesStorage {}

impl NullPagesStorage {
    /// Initialize a new `NullPagesStorage`
    pub fn new() -> Self {
        Self {}
    }
}

impl PagesStorage for NullPagesStorage {
    fn read_page(&mut self, page_idx: u32) -> Option<Vec<u8>> {
        None
    }

    fn write_page(&mut self, page_idx: u32, data: &[u8]) {}

    fn clear(&mut self) {}

    fn commit(&mut self) {}
}

/// A do nothing `PageCache`
pub type NullPageCache<'pc> = PageCache<'pc, NullPagesStorage>;

/// A do nothing `PageSliceCache`
pub type NullPageSliceCache<'pc> = PageSliceCache<'pc, NullPageCache<'pc>>;
