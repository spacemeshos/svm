use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use svm_gas::FuncPrice;
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
}

// let template_prices = template_prices.unwrap_or_default();
//
// `template_prices` offers an easy way to inject an append-only, naive caching mechanism to
// the [`Template`] pricing logic; using a `None` will result in a new
// empty cache and on-the-fly calculation for all [`Template`]s.
