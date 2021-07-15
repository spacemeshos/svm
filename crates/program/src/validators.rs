use crate::{Op, Program, ProgramVisitor};

/// A [`ProgramVisitor`] that checks whether all opcodes within a WASM program
/// satisfy a given property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpcodeValidator<F> {
    opcode_validator: F,
}

impl<F> OpcodeValidator<F> {
    pub fn new<E>(f: F) -> Self
    where
        F: Fn(&Op) -> Result<(), E>,
    {
        Self {
            opcode_validator: f,
        }
    }
}

impl<F, E> ProgramVisitor for OpcodeValidator<F>
where
    F: Fn(&Op) -> Result<(), E>,
{
    type Output = ();
    type Error = E;

    fn on_op(&mut self, op: &Op, _program: &Program) -> Result<(), Self::Error> {
        (self.opcode_validator)(op)
    }

    fn on_end(self, _program: &Program) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}
