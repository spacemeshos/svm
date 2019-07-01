use hash256_std_hasher::Hash256StdHasher;
use tiny_keccak::Keccak;

use super::traits::CodeHash;

pub struct DefaultCodeHasher;

impl hash_db::Hasher for DefaultCodeHasher {
    type Out = CodeHash;

    type StdHasher = Hash256StdHasher;

    const LENGTH: usize = 32;

    fn hash(code: &[u8]) -> CodeHash {
        let mut out = [0; 32];
        Keccak::keccak256(code, &mut out);
        out
    }
}
