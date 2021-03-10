use crate::env::{default, memory, traits};

use default::DefaultSerializers as D;
use memory::{MemAppStore, MemTemplateStore};
use traits::{EnvSerializers, EnvTypes};

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore = MemTemplateStore<
    <D as EnvSerializers>::TemplateSerializer,
    <D as EnvSerializers>::TemplateDeserializer,
>;

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore =
    MemAppStore<<D as EnvSerializers>::AppSerializer, <D as EnvSerializers>::AppDeserializer>;

pub struct DefaultMemEnvTypes;

impl EnvTypes for DefaultMemEnvTypes {
    type TemplateStore = DefaultMemTemplateStore;

    type AppStore = DefaultMemAppStore;

    type TemplateAddressCompute = default::DefaultTemplateAddressCompute;

    type AppAddressCompute = default::DefaultAppAddressCompute;

    type TemplateHasher = default::DefaultTemplateHasher;
}
