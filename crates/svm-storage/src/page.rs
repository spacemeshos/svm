/// A page is `4096 bytes`
pub const PAGE_SIZE: usize = 4096;

/// A `PageIndex` is a one-dimensional tuple of `(u32)`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageIndex(pub u32);

/// A `Page` is a tuple of `(u32, Vec<u8>)` representing `(page_index, page_content)`
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Page(pub u32, Vec<u8>);

/// A `SliceIndex` is a one-dimensional tuple of `(u32)`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SliceIndex(pub u32);

/// Defines a page-slice memory
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PageSliceLayout {
    /// The slice index
    pub slice_idx: SliceIndex,

    /// The page index the slices belong to
    pub page_idx: PageIndex,

    /// The page offset where the slice starts
    pub offset: u32,

    /// The length of the slice in bytes
    pub len: u32,
}

/// Allocates a new page (`Vec<u8>`) consisting only of zeros
#[inline(always)]
pub fn zero_page() -> Vec<u8> {
    vec![0; PAGE_SIZE]
}
