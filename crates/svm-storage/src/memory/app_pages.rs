use crate::{
    app_pages::AppPages,
    default::{DefaultPageHasher, DefaultStateHasher},
};

use svm_kv::memory::MemKVStore;

/// A `AppPages` implementation backed by `MemKVStore` kv-store.
pub type MemAppPages = AppPages<MemKVStore, DefaultPageHasher, DefaultStateHasher>;
