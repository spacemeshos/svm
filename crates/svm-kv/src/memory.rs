use crate::{key::concat_ns_to_key, traits::KVStore};

use std::collections::{hash_map, HashMap};

use log::{debug, info};

/// An implementation for a key-value store (implements `KVStore`) store backed by an underlying `HashMap`
pub struct MemKVStore {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemKVStore {
    #[allow(clippy::new_without_default)]
    /// Initializes a new `MemKVStore`
    pub fn new() -> Self {
        info!("creating a new in-memory kv");

        Self {
            map: HashMap::new(),
        }
    }

    /// Clears the key-value store
    pub fn clear(&mut self) {
        info!("clearing in-memory kv");

        self.map.clear();
    }

    /// Returns an iterator for the internal `HashMap`
    pub fn iter(&self) -> hash_map::Iter<Vec<u8>, Vec<u8>> {
        (&self.map).iter()
    }

    /// Returns an iterator over the keys
    pub fn keys(&self) -> hash_map::Keys<Vec<u8>, Vec<u8>> {
        self.map.keys()
    }
}

impl KVStore for MemKVStore {
    fn get(&self, ns: &[u8], key: &[u8]) -> Option<Vec<u8>> {
        let key = concat_ns_to_key(ns, key);

        let entry = self.map.get(&key);

        if let Some(entry) = entry {
            Some(entry.clone())
        } else {
            None
        }
    }

    fn store(&mut self, ns: &[u8], changes: &[(&[u8], &[u8])]) {
        info!("Storing in-memory kv changeset");

        for (k, v) in changes {
            let k = concat_ns_to_key(ns, k);

            self.map.insert(k, v.to_vec());
        }
    }
}

impl Drop for MemKVStore {
    fn drop(&mut self) {
        debug!("Dropping `MemKVStore`...")
    }
}
