use parity_wasm::elements::Instruction;

mod error;
mod exports;
mod function;
mod import;
mod op;
mod program;
mod validators;
mod visitor;

pub use error::ProgramError;
pub use exports::Exports;
pub use function::{FuncIndex, FuncIterator, Function};
pub use import::Imports;
pub use op::Op;
pub use program::Program;
pub use validators::OpcodeValidator;
pub use visitor::ProgramVisitor;

fn count_functions_in_program(program: &Program) -> u64 {
    #[derive(Debug, Default, Copy, Clone)]
    struct Counter(u64);

    impl ProgramVisitor for Counter {
        type Output = u64;
        type Error = ();

        fn on_func_end(
            &mut self,
            _fn_index: FuncIndex,
            _program: &Program,
        ) -> Result<(), Self::Error> {
            self.0 += 1;
            Ok(())
        }

        fn on_end(self, _program: &Program) -> Result<Self::Output, Self::Error> {
            Ok(self.0)
        }
    }

    Counter::default().visit(program).unwrap()
}

fn validate_no_floats(program: &Program) -> Result<(), ProgramError> {
    OpcodeValidator::new(validate_opcode).visit(program)
}

fn validate_opcode(op: &Op) -> Result<(), ProgramError> {
    match op.raw() {
        Instruction::F32Load(..)
        | Instruction::F64Load(..)
        | Instruction::F32Store(..)
        | Instruction::F64Store(..)
        | Instruction::F32Const(..)
        | Instruction::F64Const(..)
        | Instruction::F32Eq
        | Instruction::F32Ne
        | Instruction::F32Lt
        | Instruction::F32Gt
        | Instruction::F32Le
        | Instruction::F32Ge
        | Instruction::F64Eq
        | Instruction::F64Ne
        | Instruction::F64Lt
        | Instruction::F64Gt
        | Instruction::F64Le
        | Instruction::F64Ge
        | Instruction::F32Abs
        | Instruction::F32Neg
        | Instruction::F32Ceil
        | Instruction::F32Floor
        | Instruction::F32Trunc
        | Instruction::F32Nearest
        | Instruction::F32Sqrt
        | Instruction::F32Add
        | Instruction::F32Sub
        | Instruction::F32Mul
        | Instruction::F32Div
        | Instruction::F32Min
        | Instruction::F32Max
        | Instruction::F32Copysign
        | Instruction::F64Abs
        | Instruction::F64Neg
        | Instruction::F64Ceil
        | Instruction::F64Floor
        | Instruction::F64Trunc
        | Instruction::F64Nearest
        | Instruction::F64Sqrt
        | Instruction::F64Add
        | Instruction::F64Sub
        | Instruction::F64Mul
        | Instruction::F64Div
        | Instruction::F64Min
        | Instruction::F64Max
        | Instruction::F64Copysign
        | Instruction::F32ConvertSI32
        | Instruction::F32ConvertUI32
        | Instruction::F32ConvertSI64
        | Instruction::F32ConvertUI64
        | Instruction::F32DemoteF64
        | Instruction::F64ConvertSI32
        | Instruction::F64ConvertUI32
        | Instruction::F64ConvertSI64
        | Instruction::F64ConvertUI64
        | Instruction::F64PromoteF32
        | Instruction::F32ReinterpretI32
        | Instruction::F64ReinterpretI64 => Err(ProgramError::FloatsNotAllowed),
        _ => Ok(()),
    }
}
