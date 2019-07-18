use super::traits::KVStore;
use std::collections::HashMap;

/// An implementation for a key-value store (implements `KVStore`) store backed by an underlying `HashMap`
pub struct MemKVStore<MemKey> {
    map: HashMap<MemKey, Vec<u8>>,
}

impl<MemKey> MemKVStore<MemKey>
where
    MemKey: AsRef<[u8]> + Copy + Clone + Sized + Eq + std::hash::Hash,
{
    /// Initializes a new `MemKVStore`
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Clears the key-value store
    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<MemKey, Vec<u8>> {
        (&self.map).into_iter()
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<MemKey, Vec<u8>> {
        self.map.keys()
    }
}

impl<MemKey> KVStore for MemKVStore<MemKey>
where
    MemKey: AsRef<[u8]> + Copy + Sized + Eq + std::hash::Hash,
{
    type K = MemKey;

    fn get(&self, key: Self::K) -> Option<Vec<u8>> {
        let entry = self.map.get(&key);

        if entry.is_some() {
            Some(entry.unwrap().clone())
        } else {
            None
        }
    }

    fn store(&mut self, changes: &[(Self::K, &[u8])]) {
        for (k, v) in changes {
            self.map.insert(*k, v.to_vec());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svm_common::Address;

    #[test]
    fn a_key_does_not_exit_by_default() {
        let kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        assert_eq!(None, kv.get(addr.0));
    }

    #[test]
    fn key_store_and_then_key_get() {
        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        kv.store(&[(addr.0, &[10, 20, 30])]);
        assert_eq!(vec![10, 20, 30], kv.get(addr.0).unwrap());
    }

    #[test]
    fn key_store_override_existing_entry() {
        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);
        let change1 = (addr.0, &[10, 20, 30]);
        let change2 = (addr.0, &[40, 50, 60]);

        kv.store(&[(addr.0, &[10, 20, 30])]);
        assert_eq!(vec![10, 20, 30], kv.get(addr.0).unwrap());

        kv.store(&[(addr.0, &[40, 50, 60])]);
        assert_eq!(vec![40, 50, 60], kv.get(addr.0).unwrap());
    }

    #[test]
    fn clear() {
        let mut kv = MemKVStore::new();
        let addr1 = Address::from(0x11_22_33_44 as u32);
        let addr2 = Address::from(0x55_66_77_88 as u32);

        let change1 = (addr1.0, &vec![10, 20, 30]);
        let change2 = (addr2.0, &vec![40, 50, 60]);

        kv.store(&[(addr1.0, &[10, 20, 30]), (addr2.0, &[40, 50, 60])]);

        assert_eq!(vec![10, 20, 30], kv.get(addr1.0).unwrap());
        assert_eq!(vec![40, 50, 60], kv.get(addr2.0).unwrap());

        kv.clear();

        assert_eq!(None, kv.get(addr1.0));
        assert_eq!(None, kv.get(addr2.0));
    }
}
