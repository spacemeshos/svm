/// Generates pages-storage instance of `RocksdbPages`
#[macro_export]
macro_rules! gen_rocksdb_pages_storage {
    ($addr: expr, $state: expr, $max_pages: expr, $contract_storage_path: expr) => {{
        use std::cell::RefCell;
        use std::path::Path;
        use std::rc::Rc;

        use svm_kv::rocksdb::RocksStore;
        use svm_storage::rocksdb::RocksdbPages;

        let path = Path::new($contract_storage_path);
        let kv = RocksStore::new(path);
        let kv = Rc::new(RefCell::new(kv));

        RocksdbPages::new($addr, kv, $state, $max_pages as u32)
    }};
}

/// Wraps a `RocksdbPages` pages-storage by a page-cache instance of `RocksdbPageCache`
#[macro_export]
macro_rules! gen_rocksdb_page_cache {
    ($pages_storage: expr, $max_pages: expr) => {{
        use svm_storage::rocksdb::RocksdbPageCache;

        RocksdbPageCache::new($pages_storage, $max_pages)
    }};
}

/// Generates an environment instance of type `RocksEnv`
#[macro_export]
macro_rules! gen_rocksdb_env {
    ($code_db_path: expr) => {{
        use std::path::Path;
        use svm_contract::rocksdb::{RocksContractStore, RocksEnv};

        use svm_contract::wasm::{
            WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S,
        };

        let path = Path::new($code_db_path);
        let store = RocksContractStore::<S, D>::new(path);

        RocksEnv::new(store)
    }};
}

/// Injects `rocksdb` backed implementation of `svm` runtime.
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
            svm_storage::rocksdb::RocksdbPageCache,
            svm_contract::rocksdb::RocksEnv,
            || $crate::gen_rocksdb_env!($code_db_path)
        );
    };
}
