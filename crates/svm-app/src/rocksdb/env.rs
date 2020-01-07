use std::marker::PhantomData;

use crate::{
    default::{DefaultAppAddressCompute, DefaultAppTemplateAddressCompute, DefaultTemplateHasher},
    raw::{AppTemplateJsonDeserializer, AppTemplateJsonSerializer},
    rocksdb::{RocksdbAppStore, RocksdbAppTemplateStore},
    traits::{
        AppDeserializer, AppSerializer, AppTemplateDeserializer, AppTemplateSerializer, Env,
        EnvSerializerTypes, EnvTypes,
    },
};

pub struct RocksdbEnvTypes<Ser>(PhantomData<Ser>);

impl<Ser> EnvTypes for RocksdbEnvTypes<Ser>
where
    Ser: EnvSerializerTypes,
{
    type TemplateStore =
        RocksdbAppTemplateStore<Ser::TemplateSerializer, Ser::TemplateDeserializer>;

    type AppStore = RocksdbAppStore<Ser::AppSerializer, Ser::AppDeserializer>;

    type AppTemplateAddressCompute = DefaultAppTemplateAddressCompute;

    type AppAddressCompute = DefaultAppAddressCompute;

    type TemplateHasher = DefaultTemplateHasher;
}

/// AppTemplate environment backed-by `rocksdb`
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
    /// Creates a new `RocksdbEnv`. Injects externally the `AppTemplateStore`
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
