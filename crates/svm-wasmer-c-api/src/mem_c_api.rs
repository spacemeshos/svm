use crate::include_svm_wasmer_c_api;
use std::cell::RefCell;
use std::rc::Rc;
use svm_storage::memory::{MemKVStore, MemPageCache32, MemPages};
use svm_wasmer::*;

// Injects into this file the `svm wasmer` C-API backed by `MemKVStore, MemPages, MemPageCache32`
include_svm_wasmer_c_api!(
    |addr, _state| {
        let kv = Rc::new(RefCell::new(MemKVStore::new()));
        MemPages::new(addr, kv)
    },
    MemPageCache32
);
