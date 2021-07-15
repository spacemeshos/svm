use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use storage::StorageBuilderFn;
use svm_layout::Layout;
use svm_storage::account::{AccountKVStore, AccountStorage};
use svm_storage::kv::StatefulKV;
use svm_types::{AccountAddr, State};

use crate::{env, storage};
use crate::{Config, DefaultRuntime, Env};

use env::{DefaultRocksAccountStore, DefaultRocksEnvTypes, DefaultRocksTemplateStore};

/// Creates a new `Runtime` backed by `rocksdb` for persistence.
pub fn create_rocksdb_runtime<P>(
    state_kv: &Rc<RefCell<dyn StatefulKV>>,
    kv_path: &P,
) -> DefaultRuntime<DefaultRocksEnvTypes>
where
    P: AsRef<Path>,
{
    todo!()
    // let env = build_env(&kv_path);

    // DefaultRuntime::new(env, kv_path, storage_builder(state_kv))
}

fn build_env<P>(kv_path: &P) -> Env<DefaultRocksEnvTypes>
where
    P: AsRef<Path>,
{
    let account_store = DefaultRocksAccountStore::new(kv_path);
    let template_store = DefaultRocksTemplateStore::new(kv_path);

    Env::new(account_store, template_store)
}

pub fn storage_builder(state_kv: &Rc<RefCell<dyn StatefulKV>>) -> Box<StorageBuilderFn> {
    let state_kv = Rc::clone(state_kv);

    let func = move |addr: &AccountAddr, _state: &State, layout: &Layout, _config: &Config| {
        // The current pointed-to `State` is managed externally, so we ignore here the `state` parameter.
        //
        // Similarly, we ignore the `config` parameter since it only contains the `Path` of the key-value store
        // used managing the App's storage. We talk with the external key-value store via FFI interface.

        let addr = addr.inner();
        let account_kv = AccountKVStore::new(addr.clone(), &state_kv);

        AccountStorage::new(layout.clone(), account_kv)
    };

    Box::new(func)
}
