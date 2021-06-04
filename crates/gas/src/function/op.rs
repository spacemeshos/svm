use std::fmt::{self, Debug};

use parity_wasm::elements::Instruction;

/// An `Op` is a wrapper over a raw `Instruction` and an offset.
/// The offset is the instruction's location within the function for which the instruction belongs to.
#[derive(Clone, PartialEq, Eq)]
pub struct Op<'f> {
    pub raw: &'f Instruction,

    pub offset: usize,
}

impl<'f> Op<'f> {
    /// Creates a new `Op`
    pub fn new(raw: &'f Instruction, offset: usize) -> Self {
        Self { raw, offset }
    }

    /// Returns the wrapped raw `Instruction`
    pub fn raw(&self) -> &Instruction {
        &self.raw
    }

    /// Returns the `Op` local-offset within the function it's associated to
    pub fn offset(&self) -> usize {
        self.offset
}
}

impl<'f> Debug for Op<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.offset(), self.raw())
    }
}
