use crate::contract_pages::ContractPages;
use crate::default::{DefaultPageHasher, DefaultStateHasher};

use svm_kv::rocksdb::Rocksdb;

/// A `ContractPages` implementation backed by `Rocksdb` kv-store.
pub type RocksdbContractPages = ContractPages<Rocksdb, DefaultPageHasher, DefaultStateHasher>;
