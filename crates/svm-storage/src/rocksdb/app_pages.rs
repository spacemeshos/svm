use crate::{
    app_pages::AppPages,
    default::{DefaultPageHasher, DefaultStateHasher},
};

use svm_kv::rocksdb::Rocksdb;

/// A `AppPages` implementation backed by `Rocksdb` kv-store.
pub type RocksdbAppPages = AppPages<Rocksdb, DefaultPageHasher, DefaultStateHasher>;
