use crate::utils::u32_to_be_array;
use hash256_std_hasher::Hash256StdHasher;
use tiny_keccak::Keccak;

pub struct DefaultHasher;

impl hash_db::Hasher for DefaultHasher {
    type Out = [u8; 32];

    type StdHasher = Hash256StdHasher;

    const LENGTH: usize = 32;

    fn hash(key: &[u8]) -> Self::Out {
        let mut out = [0; Self::LENGTH];
        Keccak::keccak256(key, &mut out);
        out
    }
}

/// we introduce `hash_key` in order to avoid the need to do each time:
/// # <DefaultHasher as hash_db::Hasher>::hash(&key)
impl DefaultHasher {
    #[inline(always)]
    pub fn hash_key(key: &[u8]) -> <DefaultHasher as hash_db::Hasher>::Out {
        <DefaultHasher as hash_db::Hasher>::hash(key)
    }
}
