use std::fmt::{self, Debug};

use parity_wasm::elements::Instruction;

#[derive(Clone, PartialEq, Eq)]
pub struct Op<'f> {
    raw: &'f Instruction,

    pub offset: usize,
}

impl<'f> Op<'f> {
    pub fn new(raw: &'f Instruction, offset: usize) -> Self {
        Self { raw, offset }
    }

    pub fn raw(&self) -> &Instruction {
        &self.raw
    }

    pub fn offset(&self) -> usize {
        self.offset
}
}

impl<'f> Debug for Op<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.offset(), self.raw())
    }
}
