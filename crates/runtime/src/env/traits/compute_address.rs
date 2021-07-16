/// Computes the `Address` of an `Account / Template`.
///
/// The algorithm must be deterministic.
pub trait ComputeAddress<T> {
    type Address;

    fn compute(item: &T) -> Self::Address;
}
