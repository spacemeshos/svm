use crate::env::{self, traits};

use env::{ExtSpawnApp, ExtTemplate};
use traits::{AppAddressCompute, TemplateAddressCompute};

use svm_hash::{DefaultHasher, Hasher};
use svm_types::{Address, AppAddr, TemplateAddr};

/// Default implementation for computing an `App` address deterministically.
/// Default implementation for `TemplateAddressCompute`.
///
/// Computing the template's account address as follows:
/// Taking `Address::len()` bytes of `HASH(template.author || template.code)`
pub struct DefaultTemplateAddressCompute;

impl TemplateAddressCompute for DefaultTemplateAddressCompute {
    fn compute(template: &ExtTemplate) -> TemplateAddr {
        let cap = Address::len() + template.code().len();
        let mut buf = Vec::with_capacity(cap);

        buf.extend_from_slice(template.code());

        let hash = DefaultHasher::hash(&buf);
        let addr = Address::from(&hash[0..Address::len()]);

        TemplateAddr::new(addr)
    }
}

/// Default implementation for computing an `App Address`
pub struct DefaultAppAddressCompute;

impl AppAddressCompute for DefaultAppAddressCompute {
    fn compute(spawn: &ExtSpawnApp) -> AppAddr {
        let mut buf = Vec::with_capacity(Address::len() * 2);

        let template_addr = spawn.template_addr().inner();
        buf.extend_from_slice(template_addr.as_slice());

        let hash = DefaultHasher::hash(&buf);
        let addr = Address::from(&hash[0..Address::len()]);

        AppAddr::new(addr)
    }
}
