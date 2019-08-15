use crate::default::DefaultPageHasher;
use crate::memory::MemKVStore;
use crate::merkle_pages_storage::{KVStoreKey, MerklePagesStorage};

use svm_common::DefaultKeyHasher;

/// A `MerklePagesStorage` implementation backed b.y `MemKVStore`
pub type MemMerklePages =
    MerklePagesStorage<MemKVStore<KVStoreKey>, DefaultKeyHasher, DefaultPageHasher>;
