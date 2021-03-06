use std::marker::PhantomData;

use crate::env::{default, memory, traits};

use default::{DefaultAppAddressCompute, DefaultTemplateAddressCompute, DefaultTemplateHasher};
use memory::{MemAppStore, MemTemplateStore};
use traits::{Env, EnvSerializers, EnvTypes};

/// Aggregates the types for in-memory environments.
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

impl<S> Env for MemoryEnv<S>
where
    S: EnvSerializers,
{
    type Types = MemoryEnvTypes<S>;

    fn get_template_store(&self) -> &<Self::Types as EnvTypes>::TemplateStore {
        &self.template_store
    }

    fn get_template_store_mut(&mut self) -> &mut <Self::Types as EnvTypes>::TemplateStore {
        &mut self.template_store
    }

    fn get_app_store(&self) -> &<Self::Types as EnvTypes>::AppStore {
        &self.app_store
    }

    fn get_app_store_mut(&mut self) -> &mut <Self::Types as EnvTypes>::AppStore {
        &mut self.app_store
    }
}
