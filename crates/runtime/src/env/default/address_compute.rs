use crate::env::{self, traits};

use env::{ExtSpawn, Template};
use traits::ComputeAddress;

use svm_hash::{DefaultHasher, Hasher};
use svm_types::{Address, TemplateAddr};

/// Default implementation for computing an `Account's Address` deterministically.
///
/// Computing the template's account address as follows:
/// Taking `Address::len()` bytes of `HASH(template.deployer || template.code)`
pub struct DefaultTemplateAddressCompute;

impl ComputeAddress<Template> for DefaultTemplateAddressCompute {
    type Address = TemplateAddr;

    fn compute(template: &Template) -> TemplateAddr {
        let cap = TemplateAddr::len() + template.code().len();
        let mut buf = Vec::with_capacity(cap);

        buf.extend_from_slice(template.code());

        let hash = DefaultHasher::hash(&buf);
        let addr = TemplateAddr::from(&hash[0..TemplateAddr::len()]);

        addr
    }
}

/// Default implementation for computing an `Account's Address`
pub struct DefaultAccountAddressCompute;

impl ComputeAddress<ExtSpawn> for DefaultAccountAddressCompute {
    type Address = Address;

    fn compute(spawn: &ExtSpawn) -> Self::Address {
        let mut buf = Vec::with_capacity(Address::len() * 2);

        let template_addr = spawn.template_addr();
        buf.extend_from_slice(template_addr.as_slice());

        let hash = DefaultHasher::hash(&buf);
        let addr = Address::from(&hash[0..Address::len()]);

        addr
    }
}
