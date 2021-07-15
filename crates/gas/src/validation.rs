use parity_wasm::elements::{CustomSection, Instruction};

use svm_program::*;

use crate::{CallGraphBuilder, FixedGasError, GraphCycles};

/// Further validates a smWasm [`Program`] according to the fixed-gas rules.
///
/// The smWash [`Program`] is considered INVALID when one of the following:
///
/// * It contains the `loop` opcode.
/// * It contains the `call_indirect` opcode.
/// * It contains a call-cycles (at least one).
///   For example: function `F` calls function `G` which calls function `H` which calls again function `F`.
///   The chain of calls is: `F -> G -> H -> F`.
///
/// If none of the above occurs, then we have a valid restricted-Wasm program.
/// Otherwise, a `ProgramError` is returned.
pub fn validate_wasm(program: &Program, return_cycles: bool) -> Result<(), FixedGasError> {
    ProgramValidator::new(return_cycles).visit(&program)
}

pub struct ProgramValidator {
    current_func: Option<FuncIndex>,

    builder: CallGraphBuilder<FuncIndex>,

    return_cycles: bool,
}

impl ProgramValidator {
    pub fn new(return_cycles: bool) -> Self {
        Self {
            current_func: None,
            builder: CallGraphBuilder::new(),
            return_cycles,
        }
    }

    fn current_func(&self) -> FuncIndex {
        self.current_func.unwrap()
    }

    fn add_call(
        &mut self,
        op: &Op,
        origin: FuncIndex,
        target: FuncIndex,
    ) -> Result<(), FixedGasError> {
        if origin == target {
            return Err(FixedGasError::RecursiveCall {
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

impl ProgramVisitor for ProgramValidator {
    type Error = FixedGasError;

    type Output = ();

    fn on_start(&mut self, _program: &Program) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn on_end(mut self, _program: &Program) -> Result<Self::Output, Self::Error> {
        let call_graph = self.builder.build();

        let result = call_graph.find_cycles(self.return_cycles);

        match result {
            GraphCycles::NoCycles => Ok(()),
            GraphCycles::HasCycles(..) => Err(FixedGasError::CallCycle(result)),
        }
    }

    fn on_func_start(
        &mut self,
        fn_index: FuncIndex,
        _program: &Program,
    ) -> Result<(), Self::Error> {
        self.current_func = Some(fn_index);

        self.builder.add_target(fn_index);

        Ok(())
    }

    fn on_func_end(
        &mut self,
        _fn_index: FuncIndex,
        _program: &Program,
    ) -> Result<Self::Output, Self::Error> {
        self.current_func = None;

        Ok(())
    }

    fn on_op(&mut self, op: &Op, program: &Program) -> Result<(), Self::Error> {
        match *op.raw() {
            Instruction::Loop(..) => Err(FixedGasError::LoopNotAllowed),
            Instruction::CallIndirect(..) => Err(FixedGasError::CallIndirectNotAllowed),
            Instruction::Call(target) => {
                self.validate_func_index(target)
                    .map_err(|_| FixedGasError::LoopNotAllowed)?;

                let target = FuncIndex(target);

                if program.is_imported(target) == false {
                    let origin = self.current_func();

                    self.add_call(op, origin, target)?;
                }

                Ok(())
            }
            _ => {
                //self.validate_non_float(op)?;
                Ok(())
            }
        }
    }
}

impl ProgramValidator {}
