use crate::Layout;

/// Specifies the fixed-sized variables of an root.
pub struct LayoutBuilder {
    vars: Vec<(u32, u32)>,

    next_offset: u32,
}

/// `LayoutBuilder` represents the fixed-sized variables (storage) of an application.
impl LayoutBuilder {
    /// New builder
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

    /// Append new variables to layout.
    /// Each item of `slice` contains the corresponding variable's length.
    ///
    /// If prior to calling, `vars` had 4 items (indexed `[0..4)`) - then the first item
    /// of `slice` will map to the length of variable `4`.
    /// `slice[1]` will map to variable 5 length and so on.
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

    /// Finishes the layout building process and outputs the result `Layout`.
    pub fn build(self) -> Layout {
        Layout { vars: self.vars }
    }
}
