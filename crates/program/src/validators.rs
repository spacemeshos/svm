use crate::{Op, Program, ProgramVisitor};

/// A [`ProgramVisitor`] that checks whether all opcodes within a WASM program
/// satisfy a given property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpcodeValidator<F> {
    opcode_validator: F,
}

impl<F> OpcodeValidator<F> {
    pub fn new(f: F) -> Self
    where
        F: Fn(&Op) -> bool,
    {
        Self {
            opcode_validator: f,
        }
    }
}

impl<F> ProgramVisitor for OpcodeValidator<F>
where
    F: Fn(&Op) -> bool,
{
    type Output = ();
    type Error = usize;

    fn on_op(&mut self, op: &Op, _program: &Program) -> Result<(), Self::Error> {
        if (self.opcode_validator)(op) {
            Ok(())
        } else {
            Err(op.offset())
        }
    }

    fn on_end(self, _program: &Program) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}
