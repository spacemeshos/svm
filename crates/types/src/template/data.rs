use svm_layout::{FixedLayout, Layout};

use crate::{Section, SectionKind};

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
    pub fn with_layout(layout: Layout) -> Self {
        Self {
            layouts: vec![layout],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            layouts: Vec::with_capacity(capacity),
        }
    }

    pub fn add_layout(&mut self, layout: Layout) {
        self.layouts.push(layout);
    }

    pub fn layouts(&self) -> &[Layout] {
        &self.layouts
    }

    pub fn layout_count(&self) -> usize {
        self.layouts.len()
    }
}

impl Section for DataSection {
    const KIND: SectionKind = SectionKind::Data;
}
