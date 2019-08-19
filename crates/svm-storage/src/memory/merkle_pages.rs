use crate::default::{DefaultPageHasher, DefaultStateHasher};
use crate::memory::MemKVStore;
use crate::merkle_pages_storage::MerklePagesStorage;

/// A `MerklePagesStorage` implementation backed by `MemKVStore` kv-store.
pub type MemMerklePages = MerklePagesStorage<MemKVStore, DefaultPageHasher, DefaultStateHasher>;
