use super::{PageIndex, PageOffset, PAGE_SIZE};

/// Defines a page-slice layout (immutable structure)
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PageSliceLayout {
    /// The page index the slice belongs to
    page_idx: PageIndex,

    /// The relative-page offset where the slice starts
    offset: PageOffset,

    /// The slice length in bytes
    len: u32,
}

#[allow(clippy::len_without_is_empty)]
impl PageSliceLayout {
    /// New page-slice layout
    pub fn new(page_idx: PageIndex, offset: PageOffset, len: u32) -> Self {
        assert!(offset.0 < PAGE_SIZE);
        assert!(len < PAGE_SIZE);

        Self {
            page_idx,
            offset,
            len,
        }
    }

    /// Layout's page-index
    #[inline]
    pub fn page_index(&self) -> PageIndex {
        self.page_idx
    }

    /// Layout's page-offset
    #[inline]
    pub fn page_offset(&self) -> PageOffset {
        self.offset
    }

    /// Layout's page-length
    #[inline]
    pub fn len(&self) -> u32 {
        self.len
    }
}
