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

/// Generates an environment instance of type `RocksdbEnv`
#[macro_export]
macro_rules! gen_rocksdb_env {
    ($code_db_path: expr) => {{
        use std::path::Path;
        use svm_contract::rocksdb::{RocksdbContractStore, RocksdbEnv};

        use svm_contract::wasm::{
            WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S,
        };

        let path = Path::new($code_db_path);
        let store = RocksdbContractStore::<S, D>::new(path);

        RocksdbEnv::new(store)
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
            svm_contract::rocksdb::RocksdbEnv,
            || $crate::gen_rocksdb_env!($code_db_path)
        );
    };
}
