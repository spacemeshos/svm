use std::collections::HashMap;

/// Repersents a variable. an unsigned integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VarId(pub u32);

/// Specifies the fixed-sized variablef of an application.
#[derive(PartialEq, Clone)]
pub struct DataLayout {
    vars: HashMap<VarId, (u32, u32)>,
}

/// `DataLayout` represents the fixed-sized variables (storage) of an application.
impl DataLayout {
    /// New instance
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    /// Adds a new variable's layout
    pub fn add_var(&mut self, var_id: VarId, offset: u32, len: u32) {
        self.vars.insert(var_id, (offset, len));
    }

    /// Returns varialbe's layout. i.e: `(offset, length)`
    pub fn get_var(&self, var_id: VarId) -> (u32, u32) {
        self.vars.get(&var_id).copied().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_layout_sanity() {
        let mut layout = DataLayout::new();

        layout.add_var(VarId(0), 10, 20);
        layout.add_var(VarId(1), 30, 40);

        assert_eq!(layout.get_var(VarId(0)), (10, 20));
        assert_eq!(layout.get_var(VarId(1)), (30, 40));
    }
}
