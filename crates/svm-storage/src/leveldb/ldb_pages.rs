use crate::default::{DefaultPageHasher, DefaultStateHasher};
use crate::merkle_pages_storage::MerklePagesStorage;

use svm_kv::leveldb::LDBStore;

/// A `MerklePagesStorage` implementation backed by `LDBStore` kv-store.
pub type LDBPages = MerklePagesStorage<LDBStore, DefaultPageHasher, DefaultStateHasher>;
