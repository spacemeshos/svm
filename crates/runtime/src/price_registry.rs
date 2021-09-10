//! A collection of [`PriceRegistry`](svm_gas::PriceResolver) implementors for
//! use within a [`Runtime`](crate::Runtime).

use std::collections::HashMap;
use std::rc::Rc;

use svm_gas::{resolvers, PriceResolver};

#[derive(Clone)]
pub struct PriceResolverRegistry {
    price_resolvers: HashMap<u16, Rc<dyn PriceResolver>>,
}

impl PriceResolverRegistry {
    pub fn empty() -> Self {
        Self {
            price_resolvers: HashMap::default(),
        }
    }

    pub fn add(&mut self, version: u16, price_resolver: Rc<dyn PriceResolver>) {
        self.price_resolvers.insert(version, price_resolver);
    }

    pub fn get(&self, version: u16) -> Option<Rc<dyn PriceResolver>> {
        self.price_resolvers.get(&version).cloned()
    }
}

impl Default for PriceResolverRegistry {
    fn default() -> Self {
        let mut registry = Self::empty();
        registry.add(0, Rc::new(resolvers::V0PriceResolver::default()));
        registry
    }
}
