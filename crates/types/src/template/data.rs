use svm_layout::Layout;

use crate::{Section, SectionKind};

/// Aggregates the `Layouts` of `Template`'s Storage
///
/// This is low-level storage definitions required for executing transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    layouts: Vec<Layout>,
}

impl Default for DataSection {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl DataSection {
    /// Creates a new Section containing a single `Layout`
    pub fn with_layout(layout: Layout) -> Self {
        Self {
            layouts: vec![layout],
        }
    }

    /// Reserves room for `capacity` `Layout`s
    ///
    /// See: `add_layout`
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            layouts: Vec::with_capacity(capacity),
        }
    }

    /// Adds a `Layout`
    pub fn add_layout(&mut self, layout: Layout) {
        self.layouts.push(layout);
    }

    /// Borrows the `Layout`s of the Section
    pub fn layouts(&self) -> &[Layout] {
        &self.layouts
    }

    /// Returns the number of `Layout`s inside the Section
    pub fn len(&self) -> usize {
        self.layouts.len()
    }
}

impl Section for DataSection {
    const KIND: SectionKind = SectionKind::Data;
}
