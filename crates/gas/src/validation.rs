use crate::{CallGraph, FuncIndex, Program, ProgramError};

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
/// * It contains a chain of recursive calls.
///   For example: function `F` calls function `G` which calls function `H` which calls again function `F`.
///   The recursive chain of calls is: `F -> G -> H -> F`.
///
/// If none of the above occurs, then we have a valid restricted-Wasm program.
/// Otherwise, a `ProgramError` is returned.
pub fn validate_wasm(wasm: &[u8]) -> Result<(), ProgramError> {
    let program = crate::program_reader::read_program(wasm)?;

    let functions = program.functions();

    let mut call_graph = CallGraph::new(functions.clone());

    for &func in functions.iter() {
        validate_func(func, &program, &mut call_graph)?;
    }

    call_graph.assert_no_recursive_calls()?;

    Ok(())
}

fn validate_func(
    func: FuncIndex,
    program: &Program,
    call_graph: &mut CallGraph,
) -> Result<(), ProgramError> {
    let func_body = program.get_func_body(func).instructions();

    let _offset = validate_block(func, program, &func_body, 0, call_graph)?;

    Ok(())
}

fn validate_block(
    func: FuncIndex,
    program: &Program,
    ops: &[Instruction],
    block_offset: usize,
    call_graph: &mut CallGraph,
) -> Result<usize, ProgramError> {
    let mut offset = block_offset;

    while let Some(op) = ops.get(offset) {
        match op {
            Instruction::Loop(..) => return Err(ProgramError::LoopNotAllowed),
            Instruction::CallIndirect(..) => return Err(ProgramError::CallIndirectNotAllowed),
            &Instruction::Call(target) => {
                validate_func_index(target)?;

                let target = FuncIndex(target as u16);

                if program.is_imported(target) == false {
                    if func == target {
                        return Err(ProgramError::RecursiveCall { func, offset });
                    }

                    call_graph.add_call(func, target);
                }
                offset += 1;
            }
            Instruction::Block(..) => {
                offset = validate_block(func, program, ops, offset + 1, call_graph)?;
            }
            Instruction::If(..) => {
                let after_if = validate_block(func, program, ops, offset + 1, call_graph)?;

                if let Some(Instruction::Else) = ops.get(after_if) {
                    let after_else = validate_block(func, program, ops, after_if + 1, call_graph)?;
                    offset = after_else;
                } else {
                    offset = after_if;
                }
            }
            Instruction::Else => break,
            Instruction::End => {
                offset += 1;

                break;
            }
            _ => {
                assert_non_float(op)?;

                offset += 1;
            }
        }
    }

    Ok(offset)
}

fn validate_func_index(func: u32) -> Result<(), ProgramError> {
    if func <= std::u16::MAX as u32 {
        Ok(())
    } else {
        Err(ProgramError::FunctionIndexTooLarge)
    }
}

#[inline]
fn assert_non_float(op: &Instruction) -> Result<(), ProgramError> {
    match op {
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
