/// Computes the address of an item.
///
/// The algorithm must be deterministic.
pub trait ComputeAddress<T> {
    type Address;

    fn compute(item: &T) -> Self::Address;
}
