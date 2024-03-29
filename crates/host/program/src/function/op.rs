use std::fmt::{self, Debug};

use parity_wasm::elements::Instruction;

/// An `Op` is a wrapper over a raw `Instruction` and an offset.
/// The offset is the instruction's location within the function for which the instruction belongs to.
#[derive(Clone, PartialEq, Eq)]
pub struct Op<'f> {
    /// Reference to the `Instruction`
    pub raw: &'f Instruction,

    /// The `offset` from the start of the function
    pub offset: usize,
}

impl<'f> Debug for Op<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.offset, self.raw)
    }
}
