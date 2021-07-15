use parity_wasm::elements::Instruction;

use svm_program::{FuncIndex, OpcodeValidator, Program, ProgramVisitor};

use crate::FixedGasError;

pub fn detect_loops(program: &Program) -> Result<(), FixedGasError> {
    OpcodeValidator::new(|opcode| match opcode.raw() {
        Instruction::Loop(_) => false,
        Instruction::CallIndirect(_, _) => false,
        _ => true,
    })
    .visit(program)
    .map_err(|opcode_offset| FixedGasError::LoopNotAllowed)
}
