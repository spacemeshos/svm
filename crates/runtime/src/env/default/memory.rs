use crate::env::{default, memory};

use memory::{MemAccountStore, MemTemplateStore};

use crate::EnvTypes;

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore =
    MemTemplateStore<default::DefaultTemplateSerializer, default::DefaultTemplateDeserializer>;

/// `MemAppStore` with default serialization.
pub type DefaultMemAccountStore =
    MemAccountStore<default::DefaultAccountSerializer, default::DefaultAccountDeserializer>;

pub struct DefaultMemEnvTypes;

impl EnvTypes for DefaultMemEnvTypes {
    type TemplateStore = DefaultMemTemplateStore;

    type AccountStore = DefaultMemAccountStore;

    type TemplateAddressCompute = default::DefaultTemplateAddressCompute;

    type AccountAddressCompute = default::DefaultAccountAddressCompute;

    type TemplateHasher = default::DefaultTemplateHasher;
}
