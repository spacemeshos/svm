use super::PAGE_HASH_LEN;

/// A `PageHash` is a one-dimensional tuple of `([u8; PAGE_HASH_LEN])` representing hash of the page-content.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PageHash(pub [u8; PAGE_HASH_LEN]);

impl AsRef<[u8]> for PageHash {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<&[u8]> for PageHash {
    fn from(slice: &[u8]) -> PageHash {
        assert_eq!(
            PAGE_HASH_LEN,
            slice.len(),
            "`PageHash::from` expects exactly 32 bytes input",
        );

        let mut bytes = [0; PAGE_HASH_LEN];
        bytes.copy_from_slice(slice);

        PageHash(bytes)
    }
}
