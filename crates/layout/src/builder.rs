use crate::{FixedLayout, Id, RawVar};

/// Specifies the fixed-sized variables
pub struct LayoutBuilder {
    first: Option<Id>,

    vars: Vec<(u32, u32)>,

    next_offset: u32,
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
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
            first: None,
            vars: Vec::with_capacity(capacity),
            next_offset: 0,
        }
    }

    pub fn set_first(&mut self, first: Id) {
        self.first = Some(first);
    }

    pub fn try_first(&self) -> Option<Id> {
        self.first
    }

    pub fn first(&self) -> Id {
        self.first.unwrap()
    }

    /// Append new variables to layout.
    /// Each item of `slice` contains the corresponding variable's `byte size`.
    ///
    /// If prior to calling, `vars` had 4 items,
    /// then the first item of `slice` will map to variable `first + 4`.
    /// `slice[1]` will map to the next variable (`first + 5`) and so on.
    pub fn extend_from_slice(&mut self, slice: &[u32]) {
        for len in slice.iter() {
            self.push(*len);
        }
    }

    /// Adds a the next variable's.
    ///
    /// The `var_id` equals to the previous one plus one.
    /// The starting offset is right after where the previous variable ended.
    pub fn push(&mut self, len: u32) {
        let offset = self.next_offset;

        self.vars.push((offset, len));

        self.next_offset += len;
    }

    /// Finishes the layout building process and outputs the result `Layout`.
    pub fn build(self) -> FixedLayout {
        let first = self.first().0;

        let vars = self
            .vars
            .iter()
            .enumerate()
            .map(|(i, &(offset, byte_size))| {
                let id = Id(first + i as u32);

                RawVar::new(id, offset, byte_size)
            })
            .collect();

        FixedLayout::new(vars)
    }
}
