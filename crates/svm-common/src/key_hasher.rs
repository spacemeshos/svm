/// A low-level trait for defining a hasher
pub trait KeyHasher {
    /// `KeyHasher` produces hashes of type `Self::Hash`
    type Hash: AsRef<[u8]> + Copy + Clone + std::fmt::Debug + Sized;

    /// Receives an input `key` and returns its hash as `Self::Hash`
    fn hash(key: &[u8]) -> Self::Hash;
}
