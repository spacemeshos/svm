/// Repersents a variable. an unsigned integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VarId(pub u32);

/// Specifies the fixed-sized variables of an application.
#[derive(PartialEq, Clone)]
pub struct DataLayout {
    vars: Vec<Option<(u32, u32)>>,
}

/// `DataLayout` represents the fixed-sized variables (storage) of an application.
impl DataLayout {
    /// New instance, initialized with the total number of variables.
    pub fn new(capacity: usize) -> Self {
        Self {
            vars: vec![None; capacity],
        }
    }

    /// Adds a new variable's layout
    pub fn add_var(&mut self, var_id: VarId, offset: u32, len: u32) {
        let vid = self.var_index(var_id);

        self.vars[vid] = Some((offset, len));
    }

    /// Returns varialbe's layout. i.e: `(offset, length)`
    ///
    /// # Panics
    ///
    /// Panics when there is no layout to variable `var_id`
    pub fn get_var(&self, var_id: VarId) -> (u32, u32) {
        let vid = self.var_index(var_id);

        self.vars[vid].unwrap()
    }

    #[inline]
    fn var_index(&self, var_id: VarId) -> usize {
        let vid = var_id.0 as usize;

        assert!(vid < self.vars.capacity());

        vid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_layout_sanity() {
        let mut layout = DataLayout::new(2);

        layout.add_var(VarId(0), 10, 20);
        layout.add_var(VarId(1), 30, 40);

        assert_eq!(layout.get_var(VarId(0)), (10, 20));
        assert_eq!(layout.get_var(VarId(1)), (30, 40));
    }
}
