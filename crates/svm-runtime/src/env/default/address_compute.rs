use crate::env::traits::{AppAddressCompute, TemplateAddressCompute};

use svm_hash::{DefaultHasher, Hasher};
use svm_types::{Address, AppAddr, SpawnApp, Template, TemplateAddr};

/// Default implementation for computing an `App` address deterministically.
pub struct DefaultAppAddressCompute;

impl AppAddressCompute for DefaultAppAddressCompute {
    fn compute(spawn: &SpawnApp) -> AppAddr {
        let app = &spawn.app;

        // TODO:
        // take into account the `ctore_idx`, `ctor_buf`, `ctor_args`

        let mut buf = Vec::with_capacity(Address::len() * 2);

        let template = app.template.inner();
        buf.extend_from_slice(template.as_slice());

        let hash = DefaultHasher::hash(&buf);
        let addr = Address::from(&hash[0..Address::len()]);

        AppAddr::new(addr)
    }
}

/// Default implementation for `TemplateAddressCompute`.
///
/// Computing the template's account address as follows:
/// Taking `Address::len()` bytes of `HASH(template.author || template.code)`
pub struct DefaultTemplateAddressCompute;

impl TemplateAddressCompute for DefaultTemplateAddressCompute {
    fn compute(template: &Template) -> TemplateAddr {
        let mut buf = Vec::with_capacity(Address::len() + template.code.len());

        // TODO: extract `author` from `host_ctx`
        buf.extend_from_slice(template.code.as_slice());

        let hash = DefaultHasher::hash(&buf);
        let addr = Address::from(&hash[0..Address::len()]);

        TemplateAddr::new(addr)
    }
}
