use crate::env::traits::{AppAddressCompute, AppTemplateAddressCompute};

use svm_common::{Address, DefaultKeyHasher, KeyHasher};
use svm_types::{AppAddr, AppTemplate, HostCtx, SpawnApp, TemplateAddr};

/// Default implementation for computing an `App` address deterministically.
pub struct DefaultAppAddressCompute;

impl AppAddressCompute for DefaultAppAddressCompute {
    fn compute(spawn: &SpawnApp, _host_ctx: &HostCtx) -> AppAddr {
        let app = &spawn.app;

        // TODO:
        // take into account the `ctore_idx`, `ctor_buf`, `ctor_args`

        let mut buf = Vec::with_capacity(Address::len() * 2);

        let template = app.template.inner();
        buf.extend_from_slice(template.as_slice());

        let hash = DefaultKeyHasher::hash(&buf);
        let addr = Address::from(&hash[0..Address::len()]);

        AppAddr::new(addr)
    }
}

/// Default implementation for `AppTemplateAddressCompute`.
///
/// Computing the template's account address as follows:
/// Taking `Address::len()` bytes of `HASH(template.author || template.code)`
pub struct DefaultAppTemplateAddressCompute;

impl AppTemplateAddressCompute for DefaultAppTemplateAddressCompute {
    fn compute(template: &AppTemplate, _host_ctx: &HostCtx) -> TemplateAddr {
        let mut buf = Vec::with_capacity(Address::len() + template.code.len());

        // TODO: extract `author` from `host_ctx`
        buf.extend_from_slice(template.code.as_slice());

        let hash = DefaultKeyHasher::hash(&buf);
        let addr = Address::from(&hash[0..Address::len()]);

        TemplateAddr::new(addr)
    }
}
