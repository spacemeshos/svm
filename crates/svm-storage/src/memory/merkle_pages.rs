use crate::default::{DefaultPageHasher, DefaultStateHasher};
use crate::merkle_pages_storage::MerklePagesStorage;

use svm_kv::memory::MemKVStore;

/// A `MerklePagesStorage` implementation backed by `MemKVStore` kv-store.
pub type MemMerklePages = MerklePagesStorage<MemKVStore, DefaultPageHasher, DefaultStateHasher>;
