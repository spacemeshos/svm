/// A low-level trait for defining a hasher
pub trait KeyHasher {
    type Hash: AsRef<[u8]> + Copy + Clone + std::fmt::Debug + Sized;

    fn hash(key: &[u8]) -> Self::Hash;
}
