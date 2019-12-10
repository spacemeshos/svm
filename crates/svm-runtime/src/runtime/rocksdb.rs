use std::cell::RefCell;
use std::ffi::c_void;
use std::path::Path;
use std::rc::Rc;

use crate::opts::Opts;
use svm_common::{Address, State};
use svm_kv::rocksdb::Rocksdb;

use svm_contract::rocksdb::{RocksdbContractEnv, RocksdbContractStore};
use svm_storage::rocksdb::{RocksdbContractPageCache, RocksdbContractPages};
use svm_storage::{ContractPages, ContractStorage};

use crate::runtime::Runtime;

use wasmer_runtime_core::Func;

use svm_contract::wasm::{WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S};
use wasmer_runtime_core::import::IsExport;

// pub fn rocksdb_runtime_create(
//     vmcalls: Vec<(&'static str, IsExport)>,
// ) -> Runtime<RocksdbContractEnv> {
//     let env_builder = Box::new(rocksdb_contract_env_builder);
//     let storage_builder = Box::new(rocksdb_contract_storage_builder);
//
//     Runtime::new(vmcalls, env_builder, storage_builder)
// }

pub fn rocksdb_contract_env_builder(contracts_path: &str) -> RocksdbContractEnv {
    let path = Path::new(contracts_path);
    let store = RocksdbContractStore::<S, D>::new(path);

    RocksdbContractEnv::new(store)
}

pub fn rocksdb_contract_storage_builder(
    addr: Address,
    state: State,
    opts: &Opts,
) -> ContractStorage {
    let pages = rocksdb_contract_pages_builder(addr, state, opts);
    let page_cache = RocksdbContractPageCache::new(pages, opts.max_pages);

    ContractStorage::new(Box::new(page_cache))
}

fn rocksdb_contract_pages_builder(
    addr: Address,
    state: State,
    opts: &Opts,
) -> RocksdbContractPages {
    let path = Path::new(&opts.kv_path);
    let kv = Rocksdb::new(path);
    let kv = Rc::new(RefCell::new(kv));

    RocksdbContractPages::new(addr, kv, state, opts.max_pages as u32)
}

/// Generates pages-storage instance of `RocksdbContractPages`
#[macro_export]
macro_rules! gen_rocksdb_pages_storage {
    ($addr: expr, $state: expr, $max_pages: expr, $contract_storage_path: expr) => {{
        use std::cell::RefCell;
        use std::path::Path;
        use std::rc::Rc;

        use svm_kv::rocksdb::Rocksdb;
        use svm_storage::rocksdb::RocksdbContractPages;

        let path = Path::new($contract_storage_path);
        let kv = Rocksdb::new(path);
        let kv = Rc::new(RefCell::new(kv));

        RocksdbContractPages::new($addr, kv, $state, $max_pages as u32)
    }};
}

/// Wraps a `RocksdbContractPages` pages-storage by a page-cache instance of `RocksdbContractPageCache`
#[macro_export]
macro_rules! gen_rocksdb_page_cache {
    ($pages_storage: expr, $max_pages: expr) => {{
        use svm_storage::rocksdb::RocksdbContractPageCache;

        RocksdbContractPageCache::new($pages_storage, $max_pages)
    }};
}

/// Generates an environment instance of type `RocksdbContractEnv`
#[macro_export]
macro_rules! gen_rocksdb_env {
    ($code_db_path: expr) => {{
        use std::path::Path;
        use svm_contract::rocksdb::{RocksdbContractEnv, RocksdbContractStore};

        use svm_contract::wasm::{
            WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S,
        };

        let path = Path::new($code_db_path);
        let store = RocksdbContractStore::<S, D>::new(path);

        RocksdbContractEnv::new(store)
    }};
}

/// Injects `rocksdb` backed implementation of `SVM` runtime.
#[macro_export]
macro_rules! include_svm_rocksdb_runtime {
    ($contract_storage_path: expr, $code_db_path: expr) => {
        $crate::include_svm_runtime!(
            |addr, state, max_pages| $crate::gen_rocksdb_pages_storage!(
                addr,
                state,
                max_pages,
                $contract_storage_path
            ),
            |pages_storage, max_pages| $crate::gen_rocksdb_page_cache!(pages_storage, max_pages),
            svm_contract::rocksdb::RocksdbContractEnv,
            || $crate::gen_rocksdb_env!($code_db_path)
        );
    };
}
