use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use env::{DefaultRocksAppStore, DefaultRocksEnvTypes, DefaultRocksTemplateStore};

use crate::Env;
use crate::{env, storage};

use storage::StorageBuilderFn;

use svm_layout::Layout;
use svm_types::{AppAddr, State};

use svm_storage::app::{AppKVStore, AppStorage};
use svm_storage::kv::StatefulKV;

use crate::{Config, DefaultRuntime, ExternImport};

/// Creates a new `Runtime` backed by `rocksdb` for persistence.
pub fn create_rocksdb_runtime<P>(
    state_kv: &Rc<RefCell<dyn StatefulKV>>,
    kv_path: &P,
    imports: *const Vec<ExternImport>,
) -> DefaultRuntime<DefaultRocksEnvTypes>
where
    P: AsRef<Path>,
{
    let env = build_env(&kv_path);
    let imports = unsafe { &*imports };

    DefaultRuntime::new(env, kv_path, imports, storage_builder(state_kv))
}

fn build_env<P>(kv_path: &P) -> Env<DefaultRocksEnvTypes>
where
    P: AsRef<Path>,
{
    let app_store = DefaultRocksAppStore::new(kv_path);
    let template_store = DefaultRocksTemplateStore::new(kv_path);

    Env::new(app_store, template_store)
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
