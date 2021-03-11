use tiny_keccak::Keccak;

/// A low-level trait for defining a hasher
pub trait Hasher {
    /// `KeyHasher` produces hashes of type `Self::Hash`
    type Hash: AsRef<[u8]> + Copy + Clone + std::fmt::Debug + Sized;

    /// Receives an input `key` and returns its hash as `Self::Hash`
    fn hash(key: &[u8]) -> Self::Hash;
}

/// Implements the `KeyHasher` trait using the `keccak256` hashing algorithm (output: 32 bytes)
pub struct DefaultHasher;

impl Hasher for DefaultHasher {
    type Hash = [u8; 32];

    fn hash(key: &[u8]) -> Self::Hash {
        let mut out = [0; 32];

        Keccak::keccak256(key, &mut out);
        out
    }
}
