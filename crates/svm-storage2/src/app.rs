use std::collections::HashMap;
use std::rc::Rc;

use crate::{layout::DataLayout, raw::RawStorage};

pub struct AppStorage {
    raw_storage: Rc<RawStorage>,

    layout: DataLayout,

    uncommitted: HashMap<u32, Vec<u8>>,
}

impl AppStorage {
    pub fn new(layout: DataLayout, raw_storage: Rc<RawStorage>) -> Self {
        Self {
            layout,
            raw_storage,
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
        // ....

        self.uncommitted.clear();
    }

    #[inline]
    fn var_layout(&self, var_id: u32) -> (u32, u32) {
        self.layout.get_var(var_id)
    }
}
