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

use wasmer_runtime_core::{
    export::Export,
    import::{ImportObject, Namespace},
};

pub fn create_rocksdb_runtime(
    host: *const c_void,
    path: &str,
    exts: Vec<(String, String, Export)>,
) -> DefaultRuntime<RocksdbContractEnv> {
    let env = contract_env_build(path);

    DefaultRuntime::new(host, env, exts, Box::new(contract_storage_build))
}

fn contract_env_build(path: &str) -> RocksdbContractEnv {
    let path = Path::new(path);
    let store = RocksdbContractStore::new(path);
    RocksdbContractEnv::new(store)
}

fn contract_storage_build(
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
