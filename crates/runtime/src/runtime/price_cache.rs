use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use svm_gas::{FuncPrice, ProgramPricing};
use svm_program::{Program, ProgramVisitor};
use svm_types::TemplateAddr;

use crate::PriceResolverRegistry;

/// A naive cache for [`Template`](svm_types::Template)s' [`FuncPrice`]s.
///
/// In the future, the cache key will also include an identifier for which
/// [`PriceResolver`](svm_gas::PriceResolver) should be used (possibly an `u16`?).
pub struct TemplatePriceCache {
    registry: PriceResolverRegistry,
    cache: Rc<RefCell<HashMap<TemplateAddr, FuncPrice>>>,
}

impl TemplatePriceCache {
    pub fn new(registry: PriceResolverRegistry) -> Self {
        Self {
            registry,
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    // We're using a naive memoization mechanism: we only ever add, never remove.
    // This means there's no cache invalidation at all.
    // We can easily afford to do this because the number of [`Template`]s upon Genesis is fixed and won't grow.
    pub fn template_price(&self, template_addr: &TemplateAddr, program: &Program) -> &FuncPrice {
        let cache = self.cache.borrow_mut();

        if let Some(prices) = cache.get(&template_addr) {
            prices
        } else {
            let resolver = self.registry.get(0).expect("Missing pricing utility.");

            let pp = ProgramPricing::new(resolver);
            let prices = pp.visit(&program).unwrap();

            cache.insert(template_addr.clone(), prices);
            cache.get(template_addr).unwrap()
        }
    }
}

// let template_prices = template_prices.unwrap_or_default();
//
// `template_prices` offers an easy way to inject an append-only, naive caching mechanism to
// the [`Template`] pricing logic; using a `None` will result in a new
// empty cache and on-the-fly calculation for all [`Template`]s.
