use crate::Op;

/// Pricing an `Op` or an import function should be part of the consensus protocol.
/// (It's crucial since otherwise, different network peers will give the same function a different price).
pub trait PriceResolver {
    fn op_price(&self, op: &Op) -> usize;

    fn import_price(&self, import: (&str, &str)) -> usize;
}
