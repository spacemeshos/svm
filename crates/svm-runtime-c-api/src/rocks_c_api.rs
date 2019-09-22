crate::include_svm_runtime_c_api!(
    |addr, state, max_pages| {
        use std::cell::RefCell;
        use std::rc::Rc;
        use svm_kv::rocksdb::RocksStore;

        use svm_storage::rocksdb::RocksPages;

        let kv = Rc::new(RefCell::new(RocksStore::new(std::path::Path::new(
            "tests-contract-storage",
        ))));

        RocksPages::new(addr, kv, state, max_pages as u32)
    },
    |pages_storage, max_pages| {
        use svm_storage::rocksdb::RocksMerklePageCache;

        RocksMerklePageCache::new(pages_storage, max_pages as usize)
    },
    svm_storage::rocksdb::RocksMerklePageCache,
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
