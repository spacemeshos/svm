use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::Node;

/// Types to be used as the `label` of a `Node` needs to implement the `NodeLabel` trait
pub trait NodeLabel: Copy + Eq + Ord + Copy + Clone + Hash + Debug + Display + 'static {}

impl<L: Copy + Eq + Ord + Copy + Clone + Hash + Debug + Display + 'static> NodeLabel for L {}

fn assert_labels() {
    fn assert_label<L: NodeLabel>() {}

    assert_label::<usize>();
    assert_label::<&str>();
    assert_label::<svm_program::FuncIndex>();
}
