use std::{collections::HashMap, convert::TryFrom};

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

impl From<&DataLayout> for Vec<u8> {
    fn from(layout: &DataLayout) -> Vec<u8> {
        Vec::new()
    }
}

impl TryFrom<&[u8]> for DataLayout {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let layout = Self::new();

        Ok(layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_layout_sanity() {
        let mut layout = DataLayout::new();

        layout.add_var(0, 10, 20);
        layout.add_var(1, 30, 40);

        assert_eq!(layout.get_var(0), (10, 20));
        assert_eq!(layout.get_var(1), (30, 40));
    }
}
