use std::collections::HashMap;

use crate::{FuncIndex, Gas};

#[derive(Debug)]
pub struct FuncPrice {
    inner: HashMap<FuncIndex, Gas>,
}

impl FuncPrice {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn set_price(&mut self, func: FuncIndex, price: Gas) {
        self.inner.insert(func, price);
    }

    pub fn try_get_price(&self, func: FuncIndex) -> Option<Gas> {
        self.inner.get(&func).copied()
    }

    pub fn get_price(&self, func: FuncIndex) -> Gas {
        self.try_get_price(func).unwrap()
    }
}
