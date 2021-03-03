use std::marker::PhantomData;

use crate::{
    env::default::{
        DefaultAppAddressCompute, DefaultTemplateAddressCompute, DefaultTemplateHasher,
    },
    env::rocksdb::{RocksdbAppStore, RocksdbTemplateStore},
    env::traits::{Env, EnvSerializers, EnvTypes},
};

/// Aggregates rocksdb environment types
pub struct RocksdbEnvTypes<Ser>(PhantomData<Ser>);

impl<S> EnvTypes for RocksdbEnvTypes<S>
where
    S: EnvSerializers,
{
    type TemplateStore = RocksdbTemplateStore<S::TemplateSerializer, S::TemplateDeserializer>;

    type AppStore = RocksdbAppStore<S::AppSerializer, S::AppDeserializer>;

    type TemplateAddressCompute = DefaultTemplateAddressCompute;

    type AppAddressCompute = DefaultAppAddressCompute;

    type TemplateHasher = DefaultTemplateHasher;
}

/// `Template` environment backed-by `rocksdb`
pub struct RocksdbEnv<S>
where
    S: EnvSerializers,
{
    app_store: <RocksdbEnvTypes<S> as EnvTypes>::AppStore,

    template_store: <RocksdbEnvTypes<S> as EnvTypes>::TemplateStore,
}

impl<Ser> RocksdbEnv<Ser>
where
    Ser: EnvSerializers,
{
    /// Creates a new `RocksdbEnv`. Injects externally the `TemplateStore`
    pub fn new(
        app_store: <RocksdbEnvTypes<Ser> as EnvTypes>::AppStore,
        template_store: <RocksdbEnvTypes<Ser> as EnvTypes>::TemplateStore,
    ) -> Self {
        Self {
            app_store,
            template_store,
        }
    }
}

impl<S> Env for RocksdbEnv<S>
where
    S: EnvSerializers,
{
    type Types = RocksdbEnvTypes<S>;

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
