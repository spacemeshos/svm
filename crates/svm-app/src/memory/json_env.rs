use crate::{
    default::DefaultJsonSerializerTypes as Ser,
    memory::{MemAppStore, MemAppTemplateStore, MemoryEnv},
    traits::{AppDeserializer, AppSerializer, EnvSerializerTypes},
};

pub type JsonMemAppStore = MemAppStore<
    <Ser as EnvSerializerTypes>::AppSerializer,
    <Ser as EnvSerializerTypes>::AppDeserializer,
>;

pub type JsonMemAppTemplateStore = MemAppTemplateStore<
    <Ser as EnvSerializerTypes>::TemplateSerializer,
    <Ser as EnvSerializerTypes>::TemplateDeserializer,
>;

pub type JsonMemoryEnv = MemoryEnv<Ser>;
