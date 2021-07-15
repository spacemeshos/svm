use crate::env::{default, memory};

use memory::{MemAccountStore, MemTemplateStore};

use crate::EnvTypes;

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore =
    MemTemplateStore<default::DefaultTemplateSerializer, default::DefaultTemplateDeserializer>;

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore =
    MemAccountStore<default::DefaultAccountSerializer, default::DefaultAccountDeserializer>;

pub struct DefaultMemEnvTypes;

impl EnvTypes for DefaultMemEnvTypes {
    type TemplateStore = DefaultMemTemplateStore;

    type AccountStore = DefaultMemAppStore;

    type TemplateAddressCompute = default::DefaultTemplateAddressCompute;

    type AppAddressCompute = default::DefaultAppAddressCompute;

    type TemplateHasher = default::DefaultTemplateHasher;

    type Pricer = svm_gas::resolvers::V0PriceResolver;
}
