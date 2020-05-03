use std::collections::HashMap;

pub struct DataLayout {
    vars: HashMap<u32, (u32, u32)>,
}

impl DataLayout {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, var_id: u32, offset: u32, len: u32) {
        self.vars.insert(var_id, (offset, len));
    }

    pub fn get_var(&self, var_id: u32) -> (u32, u32) {
        self.vars.get(&var_id).copied().unwrap()
    }
}
