use crate::opts::Opts;
use crate::runtime::Runtime;

use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

use svm_contract::{
    env::{ContractEnv, ContractEnvTypes},
    memory::{MemContractStore, MemoryEnv},
};

use svm_storage::{
    memory::{MemContractPageCache, MemContractPages},
    ContractStorage,
};

pub fn create_memory_runtime(addr: Address, state: State, opts: &Opts) -> Runtime<MemoryEnv> {
    let env_builder = Box::new(memory_runtime_env_build);
    let storage_builder = Box::new(memory_runtime_storage_build);

    Runtime::new(env_builder, storage_builder)
}

fn memory_runtime_env_build(_path: &str) -> MemoryEnv {
    let store = MemContractStore::new();
    MemoryEnv::new(store)
}

fn memory_runtime_storage_build(addr: Address, state: State, opts: &Opts) -> ContractStorage {
    let kv = Rc::new(RefCell::new(MemKVStore::new()));
    let pages = MemContractPages::new(addr, kv, state, opts.max_pages as u32);
    let page_cache = MemContractPageCache::new(pages, opts.max_pages);
    let storage = ContractStorage::new(Box::new(page_cache));
    storage
}
