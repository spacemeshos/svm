use std::marker::PhantomData;

use crate::env::{default, memory, traits};

use default::{DefaultAppAddressCompute, DefaultTemplateAddressCompute, DefaultTemplateHasher};
use memory::{MemAppStore, MemTemplateStore};
use traits::{EnvSerializers, EnvTypes};

/// Aggregates in-memory environment types
pub struct MemoryEnvTypes<S>(PhantomData<S>);

impl<S> EnvTypes for MemoryEnvTypes<S>
where
    S: EnvSerializers,
{
    type TemplateStore = MemTemplateStore<S::TemplateSerializer, S::TemplateDeserializer>;

    type AppStore = MemAppStore<S::AppSerializer, S::AppDeserializer>;

    type TemplateAddressCompute = DefaultTemplateAddressCompute;

    type AppAddressCompute = DefaultAppAddressCompute;

    type TemplateHasher = DefaultTemplateHasher;
}

/// An in-memory implementation for `Env`
pub struct MemoryEnv<S>
where
    S: EnvSerializers,
{
    app_store: <MemoryEnvTypes<S> as EnvTypes>::AppStore,

    template_store: <MemoryEnvTypes<S> as EnvTypes>::TemplateStore,
}

impl<S> MemoryEnv<S>
where
    S: EnvSerializers,
{
    /// Creates a new in-memory environment.
    pub fn new(
        app_store: <MemoryEnvTypes<S> as EnvTypes>::AppStore,
        template_store: <MemoryEnvTypes<S> as EnvTypes>::TemplateStore,
    ) -> Self {
        Self {
            app_store,
            template_store,
        }
    }
}
