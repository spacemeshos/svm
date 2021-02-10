use std::marker::PhantomData;

use crate::{
    env::default::{
        DefaultAppAddressCompute, DefaultTemplateAddressCompute, DefaultTemplateHasher,
    },
    env::rocksdb::{RocksdbAppStore, RocksdbTemplateStore},
    env::traits::{Env, EnvSerializerTypes, EnvTypes},
};

pub struct RocksdbEnvTypes<Ser>(PhantomData<Ser>);

impl<Ser> EnvTypes for RocksdbEnvTypes<Ser>
where
    Ser: EnvSerializerTypes,
{
    type TemplateStore = RocksdbTemplateStore<Ser::TemplateSerializer, Ser::TemplateDeserializer>;

    type AppStore = RocksdbAppStore<Ser::AppSerializer, Ser::AppDeserializer>;

    type TemplateAddressCompute = DefaultTemplateAddressCompute;

    type AppAddressCompute = DefaultAppAddressCompute;

    type TemplateHasher = DefaultTemplateHasher;
}

/// `Template` environment backed-by `rocksdb`
pub struct RocksdbEnv<Ser>
where
    Ser: EnvSerializerTypes,
{
    app_store: <RocksdbEnvTypes<Ser> as EnvTypes>::AppStore,

    template_store: <RocksdbEnvTypes<Ser> as EnvTypes>::TemplateStore,
}

impl<Ser> RocksdbEnv<Ser>
where
    Ser: EnvSerializerTypes,
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

impl<Ser> Env for RocksdbEnv<Ser>
where
    Ser: EnvSerializerTypes,
{
    type Types = RocksdbEnvTypes<Ser>;

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
