use super::KeyHasher;

use tiny_keccak::Keccak;

/// Implements the `KeyHasher` trait using the `keccak256` hashing algorithm (output: 32 bytes)
pub struct DefaultKeyHasher;

impl KeyHasher for DefaultKeyHasher {
    type Hash = [u8; 32];

    fn hash(key: &[u8]) -> Self::Hash {
        let mut out = [0; 32];
        Keccak::keccak256(key, &mut out);
        out
    }
}
