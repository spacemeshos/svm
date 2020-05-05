use std::{cell::RefCell, rc::Rc};

use crate::kv::KV;

/// Interface against the key-value store.
/// Data is manipulated using `offset` and `length`.
pub struct RawStorage {
    kv: Rc<RefCell<dyn KV>>,
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
    pub fn new(kv: Rc<RefCell<dyn KV>>) -> Self {
        Self { kv }
    }

    /// Reads the raw data under `offset, offset + 1, ..., offset + length - 1`
    /// In case there is no stored blob, returns a zeros vector of length `length`.
    pub fn read(&self, offset: u32, length: u32) -> Vec<u8> {
        let data = self.do_read(offset, length);

        if let Some(data) = data {
            debug_assert_eq!(data.len() as u32, length);

            data
        } else {
            vec![0; length as usize]
        }
    }

    /// Write a batch of changes into underlying key-value store.
    pub fn write(&mut self, changes: &[RawChange]) {
        let changes = changes
            .iter()
            .map(|c| {
                let off = c.offset;
                let len = c.len();

                let k = self.to_key(off, len);
                let v = c.data.to_vec();

                (k, v)
            })
            .collect::<Vec<_>>();

        self.kv.borrow_mut().set(&changes);
    }

    #[inline]
    fn do_read(&self, offset: u32, length: u32) -> Option<Vec<u8>> {
        let key = self.to_key(offset, length);

        self.kv.borrow().get(&key)
    }

    #[inline]
    fn to_key(&self, offset: u32, length: u32) -> Vec<u8> {
        // built key is a concatenation of `offset` and `length`.
        // each takes exactly 4 bytes (and 8 in total).

        let mut buf = Vec::with_capacity(8);

        buf.extend_from_slice(&offset.to_be_bytes());
        buf.extend_from_slice(&length.to_be_bytes());

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::kv::StatelessKV;

    macro_rules! kv {
        () => {{
            use std::{cell::RefCell, rc::Rc};

            let kv = Rc::new(RefCell::new(StatelessKV::new()));
            kv
        }};
    }

    #[test]
    fn raw_storage_var_defaults_to_zeros() {
        let kv = kv!();

        let off = 10;
        let len = 20;

        let storage = RawStorage::new(kv);
        let bytes = storage.read(off, len);

        assert_eq!(bytes, vec![0; len as usize]);
    }

    #[test]
    fn raw_storage_store() {
        let kv = kv!();

        let var1 = RawChange {
            offset: 0,
            data: vec![0x10, 0x20, 0x30],
        };

        let var2 = RawChange {
            offset: 3,
            data: vec![0x40, 0x50],
        };

        let changes = vec![var1.clone(), var2.clone()];

        let mut storage = RawStorage::new(kv);
        storage.write(&changes);

        let data1 = storage.read(var1.offset, var1.len());
        assert_eq!(data1, vec![0x10, 0x20, 0x30]);

        let data2 = storage.read(var2.offset, var2.len());
        assert_eq!(data2, vec![0x40, 0x50]);
    }
}
