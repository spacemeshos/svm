use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use svm_kv::traits::KVStore;

use super::AppKVStore;

/// Interface against the key-value store.
/// Data is manipulated using `offset` and `length`.
pub struct RawStorage {
    app_kv: AppKVStore,

    kv_value_size: u32,

    cached_keys: HashMap<[u8; 32], Vec<u8>>,
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
    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }
}

impl RawStorage {
    /// New instance backed by key-value `kv`.
    pub fn new(app_kv: AppKVStore, kv_value_size: u32) -> Self {
        Self {
            app_kv,
            kv_value_size,
            cached_keys: HashMap::new(),
        }
    }

    /// Reads the raw data under `offset, offset + 1, ..., offset + length - 1`
    /// In case there is no stored blob, returns a zeros vector of length `length`.
    pub fn read(&self, offset: u32, length: u32) -> Vec<u8> {
        assert!(length <= self.kv_value_size);

        let key = self.read_offset_key(offset);
        debug_assert_eq!(key.len(), self.kv_value_size as usize);

        let slice = self.key_slice(&key[..], offset, length);
        slice.to_vec()
    }

    /// Write a batch of changes into underlying key-value store.
    pub fn write(&mut self, changes: &[RawChange]) {
        let change_key_idx: Vec<u32> = changes.iter().map(|c| self.change_key_index(c)).collect();

        let nchanges = changes.len();

        let keys: HashMap<u32, Vec<u8>> = (0..nchanges).fold(HashMap::new(), |mut acc, idx| {
            let key_idx = change_key_idx[idx];

            if acc.contains_key(&key_idx) {
                acc
            } else {
                let key = key_idx.to_be_bytes();
                acc.insert(key_idx, key.to_vec());

                acc
            }
        });

        let mut key_index_value: HashMap<u32, Vec<u8>> = keys
            .iter()
            .map(|(key_idx, key)| {
                let value = self.do_read_key(&key[..]);
                debug_assert_eq!(value.len(), self.kv_value_size as usize);

                (*key_idx, value)
            })
            .collect();

        for (i, c) in changes.iter().enumerate() {
            let key_index = change_key_idx[i];
            let value = key_index_value.get_mut(&key_index).unwrap();

            self.patch_value(value, c);
        }
    }

    #[inline]
    fn read_offset_key(&self, offset: u32) -> Vec<u8> {
        todo!()

        // let key = self.offset_key(offset);
        // self.do_read_key(&key[..])
    }

    #[inline]
    fn change_key_index(&self, change: &RawChange) -> u32 {
        let length = change.data.len() as u32;

        debug_assert!(length <= self.kv_value_size);

        let end_off = change.offset + length - 1;

        end_off % self.kv_value_size
    }

    #[inline]
    fn do_read_key(&self, key: &[u8]) -> Vec<u8> {
        self.app_kv
            .get(key)
            .unwrap_or(vec![0; self.kv_value_size as usize])
    }

    #[inline]
    fn key_slice<'k>(&self, key: &'k [u8], offset: u32, length: u32) -> &'k [u8] {
        let offset = offset as usize;
        let length = length as usize;

        let value = &key[offset..offset + length];
        debug_assert_eq!(value.len(), length);

        value
    }

    #[inline]
    fn patch_value(&self, value: &mut [u8], change: &RawChange) {
        unsafe {
            let src = change.data.as_ptr();
            let dst = value.as_mut_ptr().offset(change.offset as isize);
            let count = change.data.len() as usize;

            std::ptr::copy(src, dst, count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svm_common::Address;

    macro_rules! app_kv {
        ($app_addr:expr) => {{
            use crate::app::AppKVStore;
            use crate::kv::StatefulKV;

            use std::{cell::RefCell, rc::Rc};

            let raw_kv = Rc::new(RefCell::new(StatefulKV::new()));
            AppKVStore::new($app_addr, raw_kv)
        }};
    }

    #[test]
    fn raw_storage_var_defaults_to_zeros() {
        let addr = Address::of("my-app");
        let kv = app_kv!(addr);

        let off = 10;
        let len = 20;
        let kv_value_size = 32;

        let storage = RawStorage::new(kv, kv_value_size);
        let bytes = storage.read(off, len);

        assert_eq!(bytes, vec![0; len as usize]);
    }

    #[test]
    fn raw_storage_store() {
        let addr = Address::of("my-app");
        let kv = app_kv!(addr);
        let kv_value_size = 32;

        let var1 = RawChange {
            offset: 0,
            data: vec![0x10, 0x20, 0x30],
        };

        let var2 = RawChange {
            offset: 3,
            data: vec![0x40, 0x50],
        };

        let changes = vec![var1.clone(), var2.clone()];

        let mut storage = RawStorage::new(kv, kv_value_size);
        storage.write(&changes);

        let data1 = storage.read(var1.offset, var1.len());
        assert_eq!(data1, vec![0x10, 0x20, 0x30]);

        let data2 = storage.read(var2.offset, var2.len());
        assert_eq!(data2, vec![0x40, 0x50]);
    }
}
