use super::traits::KVStore;
use std::collections::HashMap;

/// An implementation for a key-value store (implements `KVStore`) store backed by an underlying `HashMap`
pub struct MemKVStore<MemKey> {
    // TODO: make `map` private and implement an `IntoIterator` for `MemKVStore`
    pub map: HashMap<MemKey, Vec<u8>>,
}

impl<MemKey> MemKVStore<MemKey>
where
    MemKey: AsRef<[u8]> + Copy + Clone + Sized + std::cmp::Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<MemKey> KVStore for MemKVStore<MemKey>
where
    MemKey: AsRef<[u8]> + Copy + Clone + Sized + std::cmp::Eq + std::hash::Hash,
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

    fn store(&mut self, key: Self::K, value: &[u8]) {
        self.map.insert(key, value.to_vec());
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

        assert_eq!(None, kv.get(addr.0));
    }

    #[test]
    fn key_store_and_then_key_get() {
        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        kv.store(addr.0, &vec![10, 20, 30]);

        assert_eq!(vec![10, 20, 30], kv.get(addr.0).unwrap());
    }

    #[test]
    fn key_store_override_existing_entry() {
        let mut kv = MemKVStore::new();
        let addr = Address::from(0x11_22_33_44 as u32);

        kv.store(addr.0, &vec![10, 20, 30]);
        assert_eq!(vec![10, 20, 30], kv.get(addr.0).unwrap());

        kv.store(addr.0, &vec![40, 50, 60]);
        assert_eq!(vec![40, 50, 60], kv.get(addr.0).unwrap());
    }

    #[test]
    fn clear() {
        let mut kv = MemKVStore::new();
        let addr1 = Address::from(0x11_22_33_44 as u32);
        let addr2 = Address::from(0x55_66_77_88 as u32);

        kv.store(addr1.0, &vec![10, 20, 30]);
        kv.store(addr2.0, &vec![40, 50, 60]);

        assert_eq!(vec![10, 20, 30], kv.get(addr1.0).unwrap());
        assert_eq!(vec![40, 50, 60], kv.get(addr2.0).unwrap());

        kv.clear();

        assert_eq!(None, kv.get(addr1.0));
        assert_eq!(None, kv.get(addr2.0));
    }
}
