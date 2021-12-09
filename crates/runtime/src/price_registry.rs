//! A collection of [`PriceRegistry`](svm_gas::PriceResolver) implementors for
//! use within a [`Runtime`](crate::Runtime).

use std::collections::HashMap;
use std::rc::Rc;

use svm_gas::{resolvers, PriceResolver};

/// A container for storing [`PriceResolver`] implementors along with their SVM
/// versions.
#[derive(Clone)]
pub struct PriceResolverRegistry {
    price_resolvers: HashMap<u16, Rc<dyn PriceResolver>>,
}

impl PriceResolverRegistry {
    /// Creates a new [`PriceResolverRegistry`] with no [`PriceResolver`].
    pub fn empty() -> Self {
        Self {
            price_resolvers: HashMap::default(),
        }
    }

    /// Stores a new [`PriceResolver`] into `self`.
    pub fn add(&mut self, version: u16, price_resolver: Rc<dyn PriceResolver>) {
        self.price_resolvers.insert(version, price_resolver);
    }

    /// Retrieves the [`PriceResolver`] associated with a certain SVM version within `self`.
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
