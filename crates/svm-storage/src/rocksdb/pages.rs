use crate::contract_pages::ContractPages;
use crate::default::{DefaultPageHasher, DefaultStateHasher};

use svm_kv::rocksdb::RocksStore;

/// A `ContractPages` implementation backed by `RocksdbStore` kv-store.
pub type RocksdbPages = ContractPages<RocksStore, DefaultPageHasher, DefaultStateHasher>;
