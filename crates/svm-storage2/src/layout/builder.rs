use super::DataLayout;

/// Specifies the fixed-sized variables of an application.
pub struct DataLayoutBuilder {
    vars: Vec<(u32, u32)>,

    next_offset: u32,
}

/// `DataLayoutBuilder` represents the fixed-sized variables (storage) of an application.
impl DataLayoutBuilder {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// New instance, initialized with the total number of variables.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vars: Vec::with_capacity(capacity),
            next_offset: 0,
        }
    }

    pub fn extend_from_slice(&mut self, slice: &[u32]) {
        for len in slice.iter() {
            self.add_var(*len);
        }
    }

    /// Adds a the next variable's.
    ///
    /// The `var_id` equals to the previous one plus one.
    /// The starting offset is right after where the previous variable ended.
    pub fn add_var(&mut self, len: u32) {
        let offset = self.next_offset;

        self.vars.push((offset, len));

        self.next_offset += len;
    }

    pub fn build(self) -> DataLayout {
        DataLayout { vars: self.vars }
    }
}
