/// A Contract's state Hash length is 32 bytes.
pub const STATE_HASH_LEN: usize = 32;

/// A `StateHash` is a one-dimensional tuple of `([u8; STATE_HASH_LEN])` representing hash of the contract state.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct StateHash(pub [u8; STATE_HASH_LEN]);

impl AsRef<[u8]> for StateHash {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
