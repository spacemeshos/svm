use crate::{
    traits::AppAddressCompute,
    types::{AppAddr, HostCtx, SpawnApp},
};

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

/// Default implementation for computing an `App` address deterministically.
pub struct DefaultAppAddressCompute;

impl AppAddressCompute for DefaultAppAddressCompute {
    fn compute(spawn: &SpawnApp, host_ctx: &HostCtx) -> AppAddr {
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
