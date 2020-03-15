use crate::{
    default::DefaultSerializerTypes as Ser,
    memory::{MemAppStore, MemAppTemplateStore, MemoryEnv},
    traits::EnvSerializerTypes,
};

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore = MemAppStore<
    <Ser as EnvSerializerTypes>::AppSerializer,
    <Ser as EnvSerializerTypes>::AppDeserializer,
>;

/// `MemAppTemplateStore` with default serialization.
pub type DefaultMemAppTemplateStore = MemAppTemplateStore<
    <Ser as EnvSerializerTypes>::TemplateSerializer,
    <Ser as EnvSerializerTypes>::TemplateDeserializer,
>;

/// `MemoryEnv` with default serialization.
pub type DefaultMemoryEnv = MemoryEnv<Ser>;
