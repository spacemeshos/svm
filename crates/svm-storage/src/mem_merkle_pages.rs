use crate::default::{DefaultPageHasher, DefaultStateHasher};
use crate::memory::MemKVStore;
use crate::merkle_pages_storage::MerklePagesStorage;

use svm_common::DefaultKeyHasher;

/// A `MerklePagesStorage` implementation backed b.y `MemKVStore`
pub type MemMerklePages = MerklePagesStorage<MemKVStore, DefaultPageHasher, DefaultStateHasher>;
