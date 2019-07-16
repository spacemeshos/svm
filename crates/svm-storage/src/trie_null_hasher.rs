use hash_db::Hasher;

/// Implements the trait `Hasher` of crate `hash-db`
/// `TrieNullHasher` doesn't do any real hashing.
/// Real hashing takes place before `reading / writing` into the trie.
pub struct TrieNullHasher;

impl Hasher for TrieNullHasher {
    type Out = [u8; 32];

    type StdHasher = hash256_std_hasher::Hash256StdHasher;

    const LENGTH: usize = 32;

    fn hash(key: &[u8]) -> Self::Out {
        assert_eq!(32, key.len());

        let mut bytes = [0; 32];
        bytes.copy_from_slice(key);

        bytes
    }
}
