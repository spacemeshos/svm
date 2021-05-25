use crate::Op;

pub trait PriceResolver {
    fn op_price(&self, op: &Op) -> usize;

    fn import_price(&self, import: (&str, &str)) -> usize;
}
