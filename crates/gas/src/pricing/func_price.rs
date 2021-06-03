use std::fmt::{self, Display};

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

impl Display for FuncPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min = self.inner.keys().min().unwrap().0;
        let max = self.inner.keys().max().unwrap().0;

        for i in min..=max {
            let fn_index = FuncIndex(i);
            let fn_price = self.get(fn_index);

            writeln!(f, "Function #{} price: {}", fn_index, fn_price)?;
        }

        Ok(())
    }
}
