use crate::traits::KVStore;
use std::collections::HashMap;

use log::info;

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
    pub fn iter(&self) -> std::collections::hash_map::Iter<Vec<u8>, Vec<u8>> {
        (&self.map).iter()
    }

    /// Returns an iterator over the keys
    pub fn keys(&self) -> std::collections::hash_map::Keys<Vec<u8>, Vec<u8>> {
        self.map.keys()
    }
}

impl KVStore for MemKVStore {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let entry = self.map.get(key);

        if let Some(entry) = entry {
            Some(entry.clone())
        } else {
            None
        }
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        info!("storing in-memory kv changeset");

        for (k, v) in changes {
            self.map.insert(k.to_vec(), v.to_vec());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svm_common::Address;

    use crate::asserts::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn a_key_does_not_exit_by_default() {
        init();

        let kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        assert_no_key!(kv, addr.as_slice());
    }

    #[test]
    fn key_store_and_then_key_get() {
        init();

        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);
        kv.store(&[(addr.as_slice(), &[10, 20, 30])]);

        assert_key_value!(kv, addr.as_slice(), vec![10, 20, 30]);
    }

    #[test]
    fn key_store_override_existing_entry() {
        init();

        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        kv.store(&[(addr.as_slice(), &[10, 20, 30])]);
        assert_key_value!(kv, addr.as_slice(), vec![10, 20, 30]);

        kv.store(&[(addr.as_slice(), &[40, 50, 60])]);
        assert_key_value!(kv, addr.as_slice(), vec![40, 50, 60]);
    }

    #[test]
    fn clear() {
        init();

        let mut kv = MemKVStore::new();
        let addr1 = Address::from(0x11_22_33_44 as u32);
        let addr2 = Address::from(0x55_66_77_88 as u32);

        kv.store(&[
            (addr1.as_slice(), &[10, 20, 30]),
            (addr2.as_slice(), &[40, 50, 60]),
        ]);

        assert_key_value!(kv, addr1.as_slice(), vec![10, 20, 30]);
        assert_key_value!(kv, addr2.as_slice(), vec![40, 50, 60]);

        kv.clear();

        assert_no_key!(kv, addr1.as_slice());
        assert_no_key!(kv, addr2.as_slice());
    }
}
