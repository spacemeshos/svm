use crate::env::{default, memory};

use memory::{MemAppStore, MemTemplateStore};

use crate::EnvTypes;

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore =
    MemTemplateStore<default::DefaultTemplateSerializer, default::DefaultTemplateDeserializer>;

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore =
    MemAppStore<default::DefaultAppSerializer, default::DefaultAppDeserializer>;

pub struct DefaultMemEnvTypes;

impl EnvTypes for DefaultMemEnvTypes {
    type TemplateStore = DefaultMemTemplateStore;

    type AppStore = DefaultMemAppStore;

    type TemplateAddressCompute = default::DefaultTemplateAddressCompute;

    type AppAddressCompute = default::DefaultAppAddressCompute;

    type TemplateHasher = default::DefaultTemplateHasher;
}
