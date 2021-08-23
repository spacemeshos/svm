use std::collections::HashMap;

use svm_types::State;

use super::AccountKVStore;
use crate::kv::StatefulKV;

/// Interface against the underling key-value store.
/// Data is manipulated using `offset` and `length`.
pub struct RawStorage {
    account_kv: AccountKVStore,
    kv_value_size: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawChange {
    /// Raw change's start offset
    pub offset: u32,

    /// Raw change's data
    pub data: Vec<u8>,
}

impl RawChange {
    /// The length of change's `data`
    #[allow(unused)]
    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }
}

impl RawStorage {
    /// New instance backed by key-value `kv`.
    pub fn new(account_kv: AccountKVStore, kv_value_size: u32) -> Self {
        Self {
            account_kv,
            kv_value_size,
        }
    }

    #[inline]
    pub fn rewind(&mut self, state: &State) {
        self.account_kv.rewind(state)
    }

    #[inline]
    pub fn head(&self) -> State {
        self.account_kv.head()
    }

    /// Reads the raw data under `offset, offset + 1, ..., offset + length - 1`
    /// In case there is no stored blob, returns a zeros vector of length `length`.
    pub fn read(&self, offset: u32, length: u32) -> Vec<u8> {
        assert!(length <= self.kv_value_size);

        let key = self.offset_length_key(offset, length);
        let value = self.do_read_key(key);

        let slice = self.value_slice(&value[..], offset, length);
        slice.to_vec()
    }

    /// Writes a batch of `RawChange` into the underlying key-value store.
    pub fn write(&mut self, changes: &[RawChange]) {
        let changes = self.group_changes_by_key(changes);

        let mut raw_changes = Vec::with_capacity(changes.len());

        for (key, value_changes) in changes.iter() {
            let raw_key = key.to_be_bytes();

            let mut raw_value = self.do_read_key(*key);
            debug_assert_eq!(raw_value.len(), self.kv_value_size as usize);

            self.patch_value(&mut raw_value, &value_changes[..]);

            raw_changes.push((raw_key, raw_value));
        }

        for (k, v) in raw_changes.iter() {
            self.account_kv.set(k, v);
        }

        let _state = self.account_kv.checkpoint();

        self.account_kv.flush();
    }

    #[inline]
    fn do_read_key(&self, key: u32) -> Vec<u8> {
        let key = key.to_be_bytes();

        self.account_kv
            .get(&key[..])
            .unwrap_or(vec![0; self.kv_value_size as usize])
    }

    #[inline]
    fn value_slice<'k>(&self, value: &'k [u8], offset: u32, length: u32) -> &'k [u8] {
        let offset = offset as usize;
        let length = length as usize;

        let value = &value[offset..offset + length];
        debug_assert_eq!(value.len(), length);

        value
    }

    #[inline]
    fn patch_value(&self, value: &mut [u8], changes: &[&RawChange]) {
        debug_assert_eq!(value.len(), self.kv_value_size as usize);

        for change in changes.iter() {
            let src = &change.data;
            let offset = change.offset as usize;
            let len = src.len();

            let dst = &mut value[offset..offset + len];

            dst.copy_from_slice(src);
        }
    }

    #[inline]
    pub fn offset_length_key(&self, offset: u32, length: u32) -> u32 {
        let end_off = offset + length - 1;

        end_off % self.kv_value_size
    }

    #[inline]
    fn change_key(&self, change: &RawChange) -> u32 {
        let length = change.data.len() as u32;
        debug_assert!(length <= self.kv_value_size);

        self.offset_length_key(change.offset, length)
    }

    #[inline]
    fn group_changes_by_key<'a>(
        &self,
        changes: &'a [RawChange],
    ) -> HashMap<u32, Vec<&'a RawChange>> {
        let tuples: Vec<_> = changes.iter().map(|c| (self.change_key(c), c)).collect();

        let mut key_changes = HashMap::new();

        for &(key, change) in tuples.iter() {
            let entry = key_changes.entry(key).or_insert(Vec::new());

            entry.push(change);
        }

        key_changes
    }
}

#[cfg(test)]
mod tests {
    use svm_types::{Address, BytesPrimitive};

    use super::*;
    use crate::testing;

    const KV_VALUE_SIZE: u32 = 32;

    #[test]
    fn raw_storage_var_defaults_to_zeros() {
        let addr = Address::of("@Account");
        let kv = testing::create_account_kv(addr);

        let off = 10;
        let len = 20;

        let storage = RawStorage::new(kv, KV_VALUE_SIZE);
        let bytes = storage.read(off, len);

        assert_eq!(bytes, vec![0; len as usize]);
    }

    #[test]
    fn raw_storage_store() {
        let addr = Address::of("@Account");
        let kv = testing::create_account_kv(addr);

        let var1 = RawChange {
            offset: 0,
            data: vec![0x10, 0x20, 0x30],
        };

        let var2 = RawChange {
            offset: 3,
            data: vec![0x40, 0x50],
        };

        let changes = vec![var1.clone(), var2.clone()];

        let mut storage = RawStorage::new(kv, KV_VALUE_SIZE);
        storage.write(&changes);

        let data1 = storage.read(var1.offset, var1.len());
        assert_eq!(data1, vec![0x10, 0x20, 0x30]);

        let data2 = storage.read(var2.offset, var2.len());
        assert_eq!(data2, vec![0x40, 0x50]);
    }
}
