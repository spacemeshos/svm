use crate::{
    traits::AppAddressCompute,
    types::{HostCtx, SpawnApp},
};

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

/// Default implementation for computing an `App` address deterministically.
pub struct DefaultAppAddressCompute;

impl AppAddressCompute for DefaultAppAddressCompute {
    fn compute(spawn: &SpawnApp, host_ctx: &HostCtx) -> Address {
        let app = &spawn.app;

        // TODO:
        // take into account the `ctore_idx`, `ctor_buf`, `ctor_args`

        let mut buf = Vec::with_capacity(Address::len() * 2);

        buf.extend_from_slice(app.template.as_slice());

        let hash = DefaultKeyHasher::hash(&buf);

        Address::from(&hash[0..Address::len()])
    }
}
