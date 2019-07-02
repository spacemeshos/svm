use super::KeyHasher;

use tiny_keccak::Keccak;

pub struct DefaultKeyHasher;

impl KeyHasher for DefaultKeyHasher {
    type Out = [u8; 32];

    fn hash(key: &[u8]) -> Self::Out {
        let mut out = [0; 32];
        Keccak::keccak256(key, &mut out);
        out
    }
}
