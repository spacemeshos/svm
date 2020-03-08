use crate::{
    traits::AppTemplateAddressCompute,
    types::{AppTemplate, HostCtx},
};

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

/// Default implementation for `AppTemplateAddressCompute`.
///
/// Computing the template's account address as follows:
/// Taking `Address::len()` bytes of `HASH(template.author || template.code)`
pub struct DefaultAppTemplateAddressCompute;

impl AppTemplateAddressCompute for DefaultAppTemplateAddressCompute {
    fn compute(template: &AppTemplate, host_ctx: &HostCtx) -> Address {
        let mut buf = Vec::with_capacity(Address::len() + template.code.len());

        // TODO: extract `author` from `host_ctx`
        buf.extend_from_slice(template.code.as_slice());

        let hash = DefaultKeyHasher::hash(&buf);

        Address::from(&hash[0..Address::len()])
    }
}
