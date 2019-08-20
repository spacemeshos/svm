use crate::default::{DefaultPageHasher, DefaultStateHasher};
use crate::leveldb::LDBStore;
use crate::merkle_pages_storage::MerklePagesStorage;

/// A `MerklePagesStorage` implementation backed by `LDBStore` kv-store.
pub type LDBPages = MerklePagesStorage<LDBStore, DefaultPageHasher, DefaultStateHasher>;
