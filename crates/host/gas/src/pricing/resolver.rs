use svm_program::Op;

/// Pricing an `Op` or an import function should be part of the consensus protocol.
/// (It's crucial since otherwise, different network peers will give the same function a different price).
pub trait PriceResolver {
    /// Returns the price for `op`
    fn op_price(&self, op: &Op) -> usize;

    /// Returns the price for import function `import`
    fn import_price(&self, import: (&str, &str)) -> usize;
}

impl<R> PriceResolver for &R
where
    R: PriceResolver,
{
    fn op_price(&self, op: &Op) -> usize {
        R::op_price(self, op)
    }

    fn import_price(&self, import: (&str, &str)) -> usize {
        R::import_price(&self, import)
    }
}
