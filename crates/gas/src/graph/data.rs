use std::fmt::{Debug, Display};

pub trait NodeData: Default + PartialEq + 'static {}

impl<D: Default + PartialEq + 'static> NodeData for D {}
