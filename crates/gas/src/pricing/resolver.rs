use crate::Op;

/// Pricing an `Op` or an import function should be part of the consensus protocol.
/// (It's crucial since otherwise, different network peers will give the same function a different price).
pub trait PriceResolver: Clone {
    /// Returns the price for `op`
    fn op_price(&self, op: &Op) -> usize;

    /// Returns the price for import function `import`
    fn import_price(&self, import: (&str, &str)) -> usize;
}
