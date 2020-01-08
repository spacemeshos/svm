use crate::{
    default::DefaultJsonSerializerTypes as Ser,
    memory::{MemAppStore, MemAppTemplateStore, MemoryEnv},
    traits::EnvSerializerTypes,
};

/// `MemAppStore` with json serialization.
pub type JsonMemAppStore = MemAppStore<
    <Ser as EnvSerializerTypes>::AppSerializer,
    <Ser as EnvSerializerTypes>::AppDeserializer,
>;

/// `MemAppTemplateStore` with json serialization.
pub type JsonMemAppTemplateStore = MemAppTemplateStore<
    <Ser as EnvSerializerTypes>::TemplateSerializer,
    <Ser as EnvSerializerTypes>::TemplateDeserializer,
>;

/// `MemoryEnv` with json serialization.
pub type JsonMemoryEnv = MemoryEnv<Ser>;
