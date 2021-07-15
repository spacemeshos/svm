use indexmap::IndexMap;

use svm_program::FuncIndex;

use std::fmt::{self, Display};

/// Stores a mapping between a function to its price
#[derive(Debug, Clone, PartialEq)]
pub struct FuncPrice {
    inner: IndexMap<FuncIndex, usize>,
}

impl FuncPrice {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates a new instance and reserve `cap` entries
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(cap),
        }
    }

    /// Sets the price `price` for function `fn_index`
    pub fn set(&mut self, fn_index: FuncIndex, price: usize) {
        self.inner.insert(fn_index, price);
    }

    /// Returns the price for function `fn_index`
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
