//! A [`Hasher`] trait for wide-digest algorithms and [`Blake3Hasher`]
//! implementation.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]

/// A low-level trait for defining a hasher.
pub trait Hasher: Default {
    /// `KeyHasher` produces hashes of type `Self::Hash`
    type Hash: AsRef<[u8]> + Copy + std::fmt::Debug + Sized;

    /// Receives an input `key` and returns its hash as `Self::Hash`.
    fn hash(key: &[u8]) -> Self::Hash {
        let mut hasher = Self::default();
        hasher.update(key);
        hasher.finalize()
    }

    /// Writes some arbitrary `bytes` into this [`Hasher`].
    fn update(&mut self, bytes: &[u8]) -> &mut Self;

    /// Returns the final [`Hasher::Hash`] value for all data written to `self`
    /// via [`Hasher::update`] so far.
    fn finalize(self) -> Self::Hash;
}

/// Implements the [`Hasher`] trait using the Blake3 hashing algorithm (output:
/// 32 bytes).
#[derive(Clone, Debug, Default)]
pub struct Blake3Hasher(blake3::Hasher);

impl std::hash::Hasher for Blake3Hasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    fn finish(&self) -> u64 {
        let mut hash = [0; 8];
        self.0.finalize_xof().fill(&mut hash);
        u64::from_be_bytes(hash)
    }
}

impl Hasher for Blake3Hasher {
    type Hash = [u8; 32];

    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.0.update(bytes);
        self
    }

    fn finalize(self) -> Self::Hash {
        *self.0.finalize().as_bytes()
    }
}
