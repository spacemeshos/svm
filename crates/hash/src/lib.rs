/// A low-level trait for defining a hasher
pub trait Hasher: Default {
    /// `KeyHasher` produces hashes of type `Self::Hash`
    type Hash: AsRef<[u8]> + Copy + Clone + std::fmt::Debug + Sized;

    /// Receives an input `key` and returns its hash as `Self::Hash`
    fn hash(key: &[u8]) -> Self::Hash {
        let mut hasher = Self::default();
        hasher.update(key);
        hasher.finalize()
    }

    fn update(&mut self, bytes: &[u8]) -> &mut Self;

    fn finalize(self) -> Self::Hash;
}

#[derive(Default)]
pub struct Blake3Hasher {
    hasher: blake3::Hasher,
}

impl Hasher for Blake3Hasher {
    type Hash = [u8; 32];

    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        self.hasher.update(bytes);
        self
    }

    fn finalize(self) -> Self::Hash {
        *self.hasher.finalize().as_bytes()
    }
}

/// Implements the `KeyHasher` trait using the `keccak256` hashing algorithm (output: 32 bytes)
pub struct DefaultHasher {
    hasher: tiny_keccak::Keccak,
}

impl Hasher for DefaultHasher {
    type Hash = [u8; 32];

    fn update(&mut self, bytes: &[u8]) -> &mut Self {
        use tiny_keccak::Hasher;

        self.hasher.update(bytes);
        self
    }

    fn finalize(self) -> Self::Hash {
        use tiny_keccak::Hasher;

        let mut out = [0; 32];
        self.hasher.finalize(&mut out);
        out
    }
}

impl Default for DefaultHasher {
    fn default() -> Self {
        Self {
            hasher: tiny_keccak::Keccak::v256(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Blake3StdHasher(blake3::Hasher);

impl std::hash::Hasher for Blake3StdHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    fn finish(&self) -> u64 {
        let mut hash = [0; 8];
        self.0.finalize_xof().fill(&mut hash);
        u64::from_be_bytes(hash)
    }
}
