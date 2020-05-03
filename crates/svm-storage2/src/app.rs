use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    kv::KV,
    layout::DataLayout,
    raw::{RawChange, RawStorage},
};

pub struct AppStorage {
    raw_storage: RawStorage,

    layout: DataLayout,

    uncommitted: HashMap<u32, Vec<u8>>,
}

impl AppStorage {
    pub fn new(layout: DataLayout, kv: Rc<RefCell<dyn KV>>) -> Self {
        Self {
            layout,
            raw_storage: RawStorage::new(kv),
            uncommitted: HashMap::new(),
        }
    }

    pub fn read_var(&self, var_id: u32) -> Vec<u8> {
        let var = self.uncommitted.get(&var_id).map(|v| v.to_owned());

        var.unwrap_or_else(|| {
            let (off, len) = self.var_layout(var_id);

            self.raw_storage.read(off, len)
        })
    }

    pub fn write_var(&mut self, var_id: u32, value: Vec<u8>) {
        self.uncommitted.insert(var_id, value);
    }

    pub fn commit(&mut self) {
        let offsets: HashMap<u32, u32> = self
            .uncommitted
            .keys()
            .map(|var_id| {
                let (off, _len) = self.var_layout(*var_id);

                (*var_id, off)
            })
            .collect();

        let changes = self
            .uncommitted
            .drain()
            .map(|(var_id, data)| {
                let offset = *offsets.get(&var_id).unwrap();

                RawChange { offset, data }
            })
            .collect::<Vec<_>>();

        self.raw_storage.store(&changes);
    }

    #[inline]
    fn var_layout(&self, var_id: u32) -> (u32, u32) {
        self.layout.get_var(var_id)
    }
}
