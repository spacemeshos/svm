use std::collections::HashMap;
use std::fmt::{Debug, Display};

use parity_wasm::elements::{Instruction, Instructions};

use crate::Gas;

/// Represents a function index
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct FuncIndex(pub u16);

impl PartialOrd for FuncIndex {
    fn partial_cmp(&self, rhs: &FuncIndex) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl Ord for FuncIndex {
    fn cmp(&self, rhs: &FuncIndex) -> std::cmp::Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl Display for FuncIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct FuncBody(pub Instructions);

impl FuncBody {
    pub fn instructions(&self) -> &[Instruction] {
        self.0.elements()
    }
}
