use std::{ffi::c_void, path::Path};

use svm_app::{
    rocksdb::{RocksdbAppStore, RocksdbAppTemplateStore, RocksdbEnv},
    traits::EnvSerializerTypes,
    types::AppAddr,
};
use svm_common::State;
use svm_storage::app::AppStorage;

use crate::{gas::GasEstimator, runtime::DefaultRuntime, settings::AppSettings};

use wasmer_runtime_core::export::Export;

/// Creates a new `Runtime` backed by `rocksdb` for persistence.
pub fn create_rocksdb_runtime<P, S, GE>(
    host: *mut c_void,
    kv_path: P,
    imports: Vec<(String, String, Export)>,
) -> DefaultRuntime<RocksdbEnv<S>, GE>
where
    P: AsRef<Path>,
    S: EnvSerializerTypes,
    GE: GasEstimator,
{
    let env = app_env_build(&kv_path);

    DefaultRuntime::new(host, env, kv_path, imports, Box::new(app_storage_build2))
}

fn app_env_build<P, S>(kv_path: &P) -> RocksdbEnv<S>
where
    P: AsRef<Path>,
    S: EnvSerializerTypes,
{
    let app_store = RocksdbAppStore::<
        <S as EnvSerializerTypes>::AppSerializer,
        <S as EnvSerializerTypes>::AppDeserializer,
    >::new(kv_path);

    let template_store = RocksdbAppTemplateStore::<
        <S as EnvSerializerTypes>::TemplateSerializer,
        <S as EnvSerializerTypes>::TemplateDeserializer,
    >::new(kv_path);

    RocksdbEnv::new(app_store, template_store)
}

fn app_storage_build2(_addr: &AppAddr, _state: &State, _settings: &AppSettings) -> AppStorage {
    todo!()
}
