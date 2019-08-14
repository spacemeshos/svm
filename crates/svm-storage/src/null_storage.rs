use crate::default::DefaultPageCache;
use crate::page::PageIndex;
use crate::traits::PagesStorage;
use crate::PageSliceCache;

/// A do-nothing `PagesStorage`.
pub struct NullPagesStorage {}

impl NullPagesStorage {
    /// Initialize a new `NullPagesStorage`
    pub fn new() -> Self {
        Self {}
    }
}

impl PagesStorage for NullPagesStorage {
    fn read_page(&mut self, _page_idx: PageIndex) -> Option<Vec<u8>> {
        None
    }

    fn write_page(&mut self, _page_idx: PageIndex, _data: &[u8]) {}

    fn clear(&mut self) {}

    fn commit(&mut self) {}
}

/// A do nothing `PageCache`
pub type NullPageCache<'pc> = DefaultPageCache<'pc, NullPagesStorage>;

/// A do nothing `PageSliceCache`
pub type NullPageSliceCache<'pc> = PageSliceCache<'pc, NullPageCache<'pc>>;
