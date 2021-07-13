/// Computes the address of an item.
///
/// The algorithm must be deterministic.
pub trait AddressLocator<T> {
    type Address;

    fn compute(item: &T) -> Self::Address;
}
