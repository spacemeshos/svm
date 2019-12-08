crate::include_svm_runtime_c_api!(
    |addr, state, max_pages| {
        use std::cell::RefCell;
        use std::rc::Rc;
        use svm_kv::rocksdb::Rocksdb;

        use svm_storage::rocksdb::RocksdbContractPages;

        let kv = Rc::new(RefCell::new(Rocksdb::new(std::path::Path::new(
            "tests-contract-storage",
        ))));

        RocksdbContractPages::new(addr, kv, state, max_pages as u32)
    },
    |pages_storage, max_pages| {
        use svm_storage::rocksdb::RocksdbContractPageCache;

        RocksdbContractPageCache::new(pages_storage, max_pages as usize)
    },
    svm_storage::rocksdb::RocksdbContractPageCache,
    svm_contract::rocksdb::RocksEnv,
    || {
        use svm_contract::{
            rocksdb::{RocksContractStore, RocksEnv},
            wasm::{WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S},
        };

        let store = RocksContractStore::<S, D>::new(std::path::Path::new("tests-contract-code"));
        RocksEnv::new(store)
    }
);
