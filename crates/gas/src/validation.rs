use crate::read::read_program;
use crate::{CallGraphBuilder, FuncIndex, Op, Program, ProgramError, ProgramVisitor};

use parity_wasm::elements::{CustomSection, Instruction};

/// Validates a Wasm program.
///
/// The wasm program is considered INVALID when one of the following:
///
/// * It contains instructions using floats.
/// * It has more than `std::u16::MAX` imported functions.
/// * The sum of imported functions and program functions exceeds `std::u16::MAX`.
/// * It contains the `loop` opcode.
/// * It contains the `call_indirect` opcode.
/// * It contains a call-cycles (at least one).
///   For example: function `F` calls function `G` which calls function `H` which calls again function `F`.
///   The chain of calls is: `F -> G -> H -> F`.
///
/// If none of the above occurs, then we have a valid restricted-Wasm program.
/// Otherwise, a `ProgramError` is returned.
pub fn validate_wasm(wasm: &[u8], return_cycles: bool) -> Result<(), ProgramError> {
    let program = read_program(wasm)?;

    let mut validator = ProgramValidator::new(&program, return_cycles);

    crate::visit_program(&program, validator)
}

pub struct ProgramValidator<'p> {
    current_func: Option<FuncIndex>,

    program: &'p Program,

    builder: CallGraphBuilder,

    return_cycles: bool,
}

impl<'p> ProgramValidator<'p> {
    pub fn new(program: &'p Program, return_cycles: bool) -> Self {
        Self {
            program,
            current_func: None,
            builder: CallGraphBuilder::new(),
            return_cycles,
        }
    }

    pub fn current_func(&self) -> FuncIndex {
        self.current_func.unwrap()
    }

    pub fn add_call(
        &mut self,
        op: &Op,
        origin: FuncIndex,
        target: FuncIndex,
    ) -> Result<(), ProgramError> {
        if origin == target {
            return Err(ProgramError::RecursiveCall {
                func: origin,
                offset: op.offset(),
            });
        }

        self.builder.add_call(origin, target);

        Ok(())
    }

    #[inline]
    fn validate_func_index(&self, func: u32) -> Result<(), ProgramError> {
        if func <= std::u16::MAX as u32 {
            Ok(())
        } else {
            Err(ProgramError::FunctionIndexTooLarge)
        }
    }
}

impl ProgramVisitor for ProgramValidator<'_> {
    type Error = ProgramError;

    fn on_start(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_end(mut self) -> Result<(), Self::Error> {
        let call_graph = self.builder.build();

        call_graph.find_cycles(self.return_cycles)
    }

    fn on_func_start(&mut self, func_index: FuncIndex) -> Result<(), Self::Error> {
        self.current_func = Some(func_index);

        Ok(())
    }

    fn on_func_end(&mut self, func_index: FuncIndex) -> Result<(), Self::Error> {
        self.current_func = None;

        Ok(())
    }

    fn on_op(&mut self, op: &Op) -> Result<(), Self::Error> {
        match *op.raw() {
            Instruction::Loop(..) => Err(ProgramError::LoopNotAllowed),
            Instruction::CallIndirect(..) => Err(ProgramError::CallIndirectNotAllowed),
            Instruction::Call(target) => {
                self.validate_func_index(target)?;

                let target = FuncIndex(target as u16);

                if self.program.is_imported(target) == false {
                    let origin = self.current_func();

                    self.add_call(op, origin, target)?;
                }

                Ok(())
            }
            _ => {
                self.validate_non_float(op)?;

                Ok(())
            }
        }
    }
}

impl ProgramValidator<'_> {
    #[inline]
    fn validate_non_float(&self, op: &Op) -> Result<(), ProgramError> {
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
}
