use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::rocksdb::Rocksdb;

use svm_contract::rocksdb::{RocksdbContractEnv, RocksdbContractStore};
use svm_storage::rocksdb::{RocksdbContractPageCache, RocksdbContractPages};
use svm_storage::ContractStorage;

use crate::contract_settings::ContractSettings;
use crate::runtime::Runtime;

pub fn create_rocksdb_runtime(path: &str) -> Runtime<RocksdbContractEnv> {
    let env = runtime_contract_env_build(path);
    let storage_builder = Box::new(runtime_contract_storage_build);

    Runtime::new(env, storage_builder)
}

fn runtime_contract_env_build(path: &str) -> RocksdbContractEnv {
    let path = Path::new(path);
    let store = RocksdbContractStore::new(path);
    RocksdbContractEnv::new(store)
}

fn runtime_contract_storage_build(
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
