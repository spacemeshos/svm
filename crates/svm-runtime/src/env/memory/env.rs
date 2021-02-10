use std::marker::PhantomData;

use crate::env::default::{
    DefaultAppAddressCompute, DefaultTemplateAddressCompute, DefaultTemplateHasher,
};

use crate::env::{
    memory::{MemAppStore, MemTemplateStore},
    traits::{Env, EnvSerializerTypes, EnvTypes},
};

pub struct MemoryEnvTypes<Ser>(PhantomData<Ser>);

impl<Ser> EnvTypes for MemoryEnvTypes<Ser>
where
    Ser: EnvSerializerTypes,
{
    type TemplateStore = MemTemplateStore<Ser::TemplateSerializer, Ser::TemplateDeserializer>;

    type AppStore = MemAppStore<Ser::AppSerializer, Ser::AppDeserializer>;

    type TemplateAddressCompute = DefaultTemplateAddressCompute;

    type AppAddressCompute = DefaultAppAddressCompute;

    type TemplateHasher = DefaultTemplateHasher;
}

/// An in-memory implementation for `Env`
pub struct MemoryEnv<Ser>
where
    Ser: EnvSerializerTypes,
{
    app_store: <MemoryEnvTypes<Ser> as EnvTypes>::AppStore,

    template_store: <MemoryEnvTypes<Ser> as EnvTypes>::TemplateStore,
}

impl<Ser> MemoryEnv<Ser>
where
    Ser: EnvSerializerTypes,
{
    /// Creates a new in-memory environment.
    pub fn new(
        app_store: <MemoryEnvTypes<Ser> as EnvTypes>::AppStore,
        template_store: <MemoryEnvTypes<Ser> as EnvTypes>::TemplateStore,
    ) -> Self {
        Self {
            app_store,
            template_store,
        }
    }
}

impl<Ser> Env for MemoryEnv<Ser>
where
    Ser: EnvSerializerTypes,
{
    type Types = MemoryEnvTypes<Ser>;

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

/// `MemoryEnv` with default serialization.
pub type DefaultMemoryEnv = MemoryEnv<crate::env::default::DefaultSerializerTypes>;
