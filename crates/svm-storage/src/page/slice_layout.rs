use super::{PageIndex, PageOffset, PAGE_SIZE};

use svm_abi::schema::VarLayout;

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

impl From<&VarLayout> for PageSliceLayout {
    fn from(layout: &VarLayout) -> Self {
        let page_idx = PageIndex(layout.page_idx as u16);
        let offset = PageOffset(layout.offset as u32);
        let len = layout.length as u32;

        PageSliceLayout::new(page_idx, offset, len)
    }
}

impl From<VarLayout> for PageSliceLayout {
    fn from(layout: VarLayout) -> Self {
        (&layout).into()
    }
}
