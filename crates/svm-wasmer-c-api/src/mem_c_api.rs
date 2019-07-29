use crate::include_svm_wasmer_c_api;
use svm_storage::memory::{MemKVStore, MemPageCache32, MemPages};
use svm_wasmer::*;

// Injects into this file the `svm wasmer` C-API backed by `MemKVStore, MemPages, MemPageCache32`
include_svm_wasmer_c_api!(MemKVStore, MemPages, MemPageCache32);
