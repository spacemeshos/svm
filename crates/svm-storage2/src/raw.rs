use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{kv::KV, layout::DataLayout};

pub struct RawStorage {
    kv: Rc<RefCell<dyn KV>>,
}

pub struct RawChange {
    pub offset: u32,

    pub data: Vec<u8>,
}

impl RawStorage {
    pub fn new(kv: Rc<RefCell<dyn KV>>) -> Self {
        Self { kv }
    }

    pub fn read(&self, offset: u32, length: u32) -> Vec<u8> {
        let data = self.do_read(offset, length);

        if let Some(data) = data {
            debug_assert_eq!(data.len() as u32, length);

            data
        } else {
            vec![0; length as usize]
        }
    }

    pub fn store(&mut self, changes: &[RawChange]) {
        //
    }

    #[inline]
    fn do_read(&self, offset: u32, length: u32) -> Option<Vec<u8>> {
        // TODO: build key
        let key = vec![0; 10];

        self.kv.borrow().get(&key)
    }
}
