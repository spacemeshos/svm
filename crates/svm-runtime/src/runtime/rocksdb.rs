use std::cell::RefCell;
use std::ffi::c_void;
use std::path::Path;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::rocksdb::Rocksdb;

use svm_contract::rocksdb::{RocksdbContractEnv, RocksdbContractStore};
use svm_storage::{
    rocksdb::{RocksdbContractPageCache, RocksdbContractPages},
    ContractStorage,
};

use crate::contract_settings::ContractSettings;
use crate::runtime::DefaultRuntime;

use wasmer_runtime_core::import::{ImportObject, Namespace};

pub fn create_rocksdb_runtime(
    host: *const c_void,
    path: &str,
) -> DefaultRuntime<RocksdbContractEnv> {
    let env = runtime_rocksdb_contract_env_build(path);

    DefaultRuntime::new(
        host,
        env,
        Box::new(runtime_rocksdb_contract_storage_build),
        Box::new(runtime_rocksdb_import_object_extender),
    )
}

fn runtime_rocksdb_contract_env_build(path: &str) -> RocksdbContractEnv {
    let path = Path::new(path);
    let store = RocksdbContractStore::new(path);
    RocksdbContractEnv::new(store)
}

fn runtime_rocksdb_contract_storage_build(
    addr: &Address,
    state: &State,
    settings: &ContractSettings,
) -> ContractStorage {
    let path = Path::new(&settings.kv_path);
    let kv = Rc::new(RefCell::new(Rocksdb::new(path)));

    let pages = RocksdbContractPages::new(addr.clone(), kv, state.clone(), settings.pages_count);
    let cache = RocksdbContractPageCache::new(pages, settings.pages_count);

    ContractStorage::new(Box::new(cache))
}

pub fn runtime_rocksdb_import_object_extender(import_object: &mut ImportObject) {
    let mut ns = Namespace::new();
    crate::vmcalls::insert_vmcalls(&mut ns);

    import_object.register("svm", ns);
}
