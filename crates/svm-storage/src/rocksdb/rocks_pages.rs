use crate::default::{DefaultPageHasher, DefaultStateHasher};
use crate::merkle_pages_storage::MerklePagesStorage;

use svm_kv::rocksdb::RocksStore;

/// A `MerklePagesStorage` implementation backed by `RocksdbStore` kv-store.
pub type RocksPages = MerklePagesStorage<RocksStore, DefaultPageHasher, DefaultStateHasher>;
