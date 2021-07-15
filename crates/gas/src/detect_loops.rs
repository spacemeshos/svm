use parity_wasm::elements::Instruction;

use svm_program::{FuncIndex, OpcodeValidator, Program, ProgramVisitor};

use crate::FixedGasError;

pub fn detect_loops(program: &Program) -> Result<(), FixedGasError> {
    OpcodeValidator::new(|opcode| match opcode.raw() {
        Instruction::Loop(_) => Err(FixedGasError::LoopNotAllowed),
        Instruction::CallIndirect(_, _) => Err(FixedGasError::CallIndirectNotAllowed),
        _ => Ok(()),
    })
    .visit(program)
}
