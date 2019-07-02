use hash256_std_hasher::Hash256StdHasher;
use tiny_keccak::Keccak;

pub struct DefaultHasher;

impl hash_db::Hasher for DefaultHasher {
    type Out = [u8; 32];

    type StdHasher = Hash256StdHasher;

    const LENGTH: usize = 32;

    fn hash(code: &[u8]) -> Self::Out {
        let mut out = [0; Self::LENGTH];
        Keccak::keccak256(code, &mut out);
        out
    }
}
