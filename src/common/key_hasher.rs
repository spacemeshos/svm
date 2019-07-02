/// A low-level trait for defining an hasher
pub trait KeyHasher {
    type Out: AsRef<[u8]> + Copy + Clone + std::fmt::Debug;

    fn hash(key: &[u8]) -> Self::Out;
}
