mod hash;
mod joined_pages_hash;
mod slice_layout;

pub use hash::PageHash;
pub use joined_pages_hash::JoinedPagesHash;
pub use slice_layout::PageSliceLayout;

/// A page is `4096 bytes`
pub const PAGE_SIZE: u32 = 4_096;

/// `PageHash` length is 32 bytes
pub const PAGE_HASH_LEN: usize = 32;

/// A `PageIndex` represents a page-index (non-negative integer)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageIndex(pub u16);

/// A `PageOffset` represents a page-offset (non-negative integer)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageOffset(pub u32);

/// A `Page` consists of a tuple of `(PageIndex, PageHash, Vec<u8>`)`
///
/// `PageIndex` - The page index within the app-storage.
/// `PageHash`  - Hash of the page. Derived from `PageIndex` + `Page Data`.
///               See also: `PageHasher` under `traits`
/// `Vec<u8>`   - The page data
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Page(pub PageIndex, pub PageHash, pub Vec<u8>);

/// Allocates a new page (`Vec<u8>`) consisting of only of zeros
#[inline]
pub fn zero_page() -> Vec<u8> {
    vec![0; PAGE_SIZE as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "`PageHash::from` expects exactly 32 bytes input")]
    fn page_hash_expects_exactly_32_bytes_input() {
        PageHash::from([0; 10].as_ref());
    }

    #[test]
    fn page_hash_from_slice() {
        let raw: [u8; 32] = [
            01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33, 44,
            55, 66, 77, 88, 99, 251, 252, 253, 254, 255,
        ];

        let ph = PageHash::from(raw.as_ref());

        assert_eq!(
            PageHash([
                01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33,
                44, 55, 66, 77, 88, 99, 251, 252, 253, 254, 255
            ]),
            ph
        );
    }
}
