use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::env::rocksdb::{RocksdbAppStore, RocksdbEnv, RocksdbTemplateStore};
use crate::env::traits::EnvSerializers;
use crate::storage::StorageBuilderFn;

use svm_layout::Layout;
use svm_types::{AppAddr, State};

use svm_storage::app::{AppKVStore, AppStorage};
use svm_storage::kv::StatefulKV;

use crate::gas::GasEstimator;
use crate::{Config, DefaultRuntime, ExternImport};

/// Creates a new `Runtime` backed by `rocksdb` for persistence.
pub fn create_rocksdb_runtime<P, S, GE>(
    state_kv: &Rc<RefCell<dyn StatefulKV>>,
    kv_path: P,
    imports: *const Vec<ExternImport>,
) -> DefaultRuntime<RocksdbEnv<S>, GE>
where
    P: AsRef<Path>,
    S: EnvSerializers,
    GE: GasEstimator,
{
    let env = build_env(&kv_path);
    let imports = unsafe { &*imports };

    DefaultRuntime::new(env, kv_path, imports, storage_builder(state_kv))
}

fn build_env<P, S>(kv_path: &P) -> RocksdbEnv<S>
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

pub fn storage_builder(state_kv: &Rc<RefCell<dyn StatefulKV>>) -> Box<StorageBuilderFn> {
    let state_kv = Rc::clone(state_kv);

    let func = move |addr: &AppAddr, _state: &State, layout: &Layout, _config: &Config| {
        // The current pointed-to `State` is managed externally, so we ignore here the `state` parameter.
        //
        // Similarly, we ignore the `config` parameter since it only contains the `Path` of the key-value store
        // used managing the App's storage. We talk with the external key-value store via FFI interface.

        let addr = addr.inner();
        let app_kv = AppKVStore::new(addr.clone(), &state_kv);

        AppStorage::new(layout.clone(), app_kv)
    };

    Box::new(func)
}
