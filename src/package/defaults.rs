use hash256_std_hasher::Hash256StdHasher;
use tiny_keccak::Keccak;

use super::code_hash::{CodeHash, CODE_HASH_LENGTH};

pub struct DefaultCodeHasher;

impl hash_db::Hasher for DefaultCodeHasher {
    type Out = CodeHash;

    type StdHasher = Hash256StdHasher;

    const LENGTH: usize = CODE_HASH_LENGTH;

    fn hash(code: &[u8]) -> CodeHash {
        let mut out = [0; 32];
        Keccak::keccak256(code, &mut out);
        out
    }
}
