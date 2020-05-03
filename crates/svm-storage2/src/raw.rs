use std::collections::HashMap;
use std::rc::Rc;

use crate::{kv::KV, layout::DataLayout};

pub struct RawStorage {
    kv: Rc<dyn KV>,
}

pub struct RawChange {
    offset: u32,

    data: u32,
}

impl RawStorage {
    pub fn new(kv: Rc<dyn KV>) -> Self {
        Self { kv }
    }

    pub fn read(&self, offset: u32, length: u32) -> Vec<u8> {
        todo!()
    }

    pub fn store(&mut self, changes: &[RawChange]) {
        //
    }
}
