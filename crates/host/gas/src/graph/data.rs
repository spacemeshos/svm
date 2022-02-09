use std::fmt::{Debug, Display};

/// Types to be used as the `data` of a `Node` needs to implement the `NodeData` trait
pub trait NodeData: Default + PartialEq + 'static {}

impl<D: Default + PartialEq + 'static> NodeData for D {}
