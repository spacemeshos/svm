use svm_storage::memory::MemMerklePageCache;

crate::include_svm_wasmer_c_api!(
    |addr, state, max_pages| {
        use std::cell::RefCell;
        use std::rc::Rc;

        use svm_storage::memory::{MemKVStore, MemMerklePages};

        let kv = Rc::new(RefCell::new(MemKVStore::new()));
        MemMerklePages::new(addr, kv, state, max_pages)
    },
    |pages_storage, max_pages| {
        use svm_storage::memory::MemMerklePageCache;

        MemMerklePageCache::new(pages_storage, max_pages)
    },
    MemMerklePageCache,
    svm_contract::memory::MemoryEnv,
    || {
        use svm_contract::{
            memory::{MemContractStore, MemoryEnv},
            wasm::{WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S},
        };

        let store = MemContractStore::<S, D>::new();
        MemoryEnv::new(store)
    }
);
