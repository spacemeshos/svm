use std::path::Path;

use crate::env::rocksdb::{RocksdbAppStore, RocksdbEnv, RocksdbTemplateStore};
use crate::env::traits::EnvSerializers;

use svm_layout::Layout;
use svm_storage::app::AppStorage;
use svm_types::{AppAddr, State};

use crate::{gas::GasEstimator, runtime::DefaultRuntime, Config, ExternImport};

/// Creates a new `Runtime` backed by `rocksdb` for persistence.
pub fn create_rocksdb_runtime<P, S, GE>(
    kv_path: P,
    imports: *const Vec<ExternImport>,
) -> DefaultRuntime<RocksdbEnv<S>, GE>
where
    P: AsRef<Path>,
    S: EnvSerializers,
    GE: GasEstimator,
{
    let env = app_env_build(&kv_path);
    let imports = unsafe { &*imports };

    DefaultRuntime::new(env, kv_path, imports, Box::new(app_storage_build))
}

fn app_env_build<P, S>(kv_path: &P) -> RocksdbEnv<S>
where
    P: AsRef<Path>,
    S: EnvSerializers,
{
    let app_store = RocksdbAppStore::<
        <S as EnvSerializers>::AppSerializer,
        <S as EnvSerializers>::AppDeserializer,
    >::new(kv_path);

    let template_store = RocksdbTemplateStore::<
        <S as EnvSerializers>::TemplateSerializer,
        <S as EnvSerializers>::TemplateDeserializer,
    >::new(kv_path);

    RocksdbEnv::new(app_store, template_store)
}

fn app_storage_build(
    _addr: &AppAddr,
    _state: &State,
    _layout: &Layout,
    _config: &Config,
) -> AppStorage {
    todo!()
}
