use crate::{
    traits::AppTemplateAddressCompute,
    types::{DeployAppTemplate, HostCtx},
};

use svm_common::{Address, DefaultKeyHasher, KeyHasher};

/// Default implementation for `AppTemplateAddressCompute`.
///
/// Computing the template's account address as follows:
/// Taking `Address::len()` bytes of `HASH(template.author || template.code)`
pub struct DefaultAppTemplateAddressCompute;

impl AppTemplateAddressCompute for DefaultAppTemplateAddressCompute {
    fn compute(deploy_template: &DeployAppTemplate, host_ctx: &HostCtx) -> Address {
        let template = &deploy_template.template;

        let mut buf = Vec::with_capacity(Address::len() + template.code.len());

        buf.extend_from_slice(deploy_template.author.as_slice());
        buf.extend_from_slice(template.code.as_slice());

        let hash = DefaultKeyHasher::hash(&buf);

        Address::from(&hash[0..Address::len()])
    }
}
