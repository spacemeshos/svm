use crate::include_svm_wasmer_c_api;
use std::cell::RefCell;
use std::rc::Rc;

use svm_storage::memory::{MemKVStore, MemMerklePageCache, MemMerklePages};
use svm_wasmer::*;

include_svm_wasmer_c_api!(
    |addr, state, max_pages| {
        let kv = Rc::new(RefCell::new(MemKVStore::new()));
        MemMerklePages::new(addr, kv, state, max_pages)
    },
    MemMerklePageCache,
    svm_contract::memory::MemoryEnv,
    || {
        use svm_contract::wasm::{
            WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S,
        };

        let store = svm_contract::memory::MemContractStore::<S, D>::new();
        svm_contract::memory::MemoryEnv::new(store)
    }
);
