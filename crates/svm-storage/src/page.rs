/// A page is `4096 bytes`
pub const PAGE_SIZE: usize = 4096;

/// A `PageIndex` is a one-dimensional tuple of `(u32)`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PageIndex(pub u32);

/// A `SliceIndex` is a one-dimensional tuple of `(u32)`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SliceIndex(pub u32);

/// Allocates a new page (`Vec<u8>`) consisting only of zeros
#[inline(always)]
pub fn zero_page() -> Vec<u8> {
    vec![0; PAGE_SIZE]
}
