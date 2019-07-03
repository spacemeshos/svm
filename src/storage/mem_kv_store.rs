use super::traits::KVStore;
use std::collections::HashMap;

pub struct MemKVStore {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemKVStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl KVStore for MemKVStore {
    type K = [u8; 32];

    fn get(&self, key: Self::K) -> Vec<u8> {
        let entry = self.map.get(key.as_ref());

        if entry.is_some() {
            entry.unwrap().clone()
        } else {
            Vec::new()
        }
    }

    fn store(&mut self, key: Self::K, value: &[u8]) {
        self.map.insert(key.to_vec(), value.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Address;

    #[test]
    fn a_key_does_not_exit_by_default() {
        let kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        assert_eq!(Vec::<u8>::new(), kv.get(addr.0));
    }

    #[test]
    fn key_store_and_then_key_get() {
        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        kv.store(addr.0, &vec![10, 20, 30]);

        assert_eq!(vec![10, 20, 30], kv.get(addr.0));
    }

    #[test]
    fn key_store_override_existing_entry() {
        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        kv.store(addr.0, &vec![10, 20, 30]);
        assert_eq!(vec![10, 20, 30], kv.get(addr.0));

        kv.store(addr.0, &vec![40, 50, 60]);
        assert_eq!(vec![40, 50, 60], kv.get(addr.0));
    }
}
