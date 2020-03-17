use std::{cell::RefCell, ffi::c_void, path::Path, rc::Rc};

use svm_common::{Address, State};
use svm_kv::rocksdb::Rocksdb;

use svm_app::{
    rocksdb::{RocksdbAppStore, RocksdbAppTemplateStore, RocksdbEnv},
    traits::EnvSerializerTypes,
    types::AppAddr,
};

use svm_storage::{
    rocksdb::{RocksdbAppPageCache, RocksdbAppPages},
    AppStorage,
};

use crate::{gas::GasEstimator, runtime::DefaultRuntime, settings::AppSettings};

use wasmer_runtime_core::export::Export;

/// Creates a new `Runtime` backed by `rocksdb` for persistence.
pub fn create_rocksdb_runtime<P, Ser, GE>(
    host: *mut c_void,
    path: &P,
    imports: Vec<(String, String, Export)>,
) -> DefaultRuntime<RocksdbEnv<Ser>, GE>
where
    P: AsRef<Path>,
    Ser: EnvSerializerTypes,
    GE: GasEstimator,
{
    let env = app_env_build(path);

    DefaultRuntime::new(host, env, imports, Box::new(app_storage_build))
}

fn app_env_build<P, Ser>(path: &P) -> RocksdbEnv<Ser>
where
    P: AsRef<Path>,
    Ser: EnvSerializerTypes,
{
    let app_store = RocksdbAppStore::<
        <Ser as EnvSerializerTypes>::AppSerializer,
        <Ser as EnvSerializerTypes>::AppDeserializer,
    >::new(path);

    let template_store = RocksdbAppTemplateStore::<
        <Ser as EnvSerializerTypes>::TemplateSerializer,
        <Ser as EnvSerializerTypes>::TemplateDeserializer,
    >::new(path);

    RocksdbEnv::new(app_store, template_store)
}

fn app_storage_build(addr: &AppAddr, state: &State, settings: &AppSettings) -> AppStorage {
    // TODO: inject path
    let path = Path::new("apps");

    let kv = Rc::new(RefCell::new(Rocksdb::new(path)));

    let pages = RocksdbAppPages::new(addr.inner().clone(), kv, state.clone(), settings.page_count);
    let cache = RocksdbAppPageCache::new(pages, settings.page_count);

    AppStorage::new(Box::new(cache))
}
