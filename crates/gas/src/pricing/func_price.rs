use indexmap::IndexMap;

use crate::FuncIndex;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncPrice {
    inner: IndexMap<FuncIndex, usize>,
}

impl FuncPrice {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(cap),
        }
    }

    pub fn set(&mut self, fn_index: FuncIndex, price: usize) {
        self.inner.insert(fn_index, price);
    }

    pub fn get(&self, fn_index: FuncIndex) -> usize {
        *self.inner.get(&fn_index).unwrap()
    }
}
