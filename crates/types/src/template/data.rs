use svm_layout::{FixedLayout, Layout};

#[derive(Clone)]
pub struct DataSection {
    layout: Layout,
}

impl DataSection {
    pub fn new(fixed: FixedLayout) -> Self {
        let layout = Layout::Fixed(fixed);

        Self { layout }
    }

    pub fn fixed_layout(&self) -> &FixedLayout {
        self.layout.as_fixed()
    }
}
