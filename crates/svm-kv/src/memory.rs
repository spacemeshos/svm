use crate::traits::RawKV;

use std::collections::{hash_map, HashMap};

use log::{debug, info};

/// An implementation for a key-value store (implements `RawKV`) store backed by an underlying `HashMap`.
pub struct MemRawKV {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemRawKV {
    #[allow(clippy::new_without_default)]
    /// Initializes a new `MemRawKV`
    pub fn new() -> Self {
        info!("creating a new in-memory raw-kv");

        Self {
            map: HashMap::new(),
        }
    }

    /// Clears the key-value store
    pub fn clear(&mut self) {
        info!("clearing in-memory raw-kv");

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

impl RawKV for MemRawKV {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key = key.to_vec();

        self.map.get(&key).cloned()
    }

    fn set(&mut self, changes: &[(&[u8], &[u8])]) {
        info!("Stores in-memory raw-kv pending changes");

        for (k, v) in changes {
            let k = k.to_vec();
            let v = v.as_ref().to_vec();

            self.map.insert(k, v);
        }
    }
}

impl Drop for MemRawKV {
    fn drop(&mut self) {
        debug!("Dropping `MemRawKV`...")
    }
}
