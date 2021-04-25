use crate::{call_graph::CallGraph, error::ProgramError, function::FuncIndex, program::Program};

use parity_wasm::elements::Instruction;

/// Validates a Wasm program.
///
/// The wasm program is considered invalid when one of the following:
///
/// * It contains instructions using floats.
/// * It has more than `std::u16::MAX` imported functions.
/// * The sum of imported functions and program functions exceeds `std::u16::MAX`.
/// * It contains `loop`  
/// * It contains `call_indirect`
/// * It contains a chain of recursive calls.
///   For example: function `F` calls function `G` which calls function `H` which calls again function `F`.
///   The recursive chain call is: `F -> G -> H -> F`.
pub fn validate_code(wasm: &[u8]) -> Result<(), ProgramError> {
    let program = crate::program_reader::read_program(wasm)?;

    let funcs_ids = program.functions();
    let mut call_graph = CallGraph::new(funcs_ids.clone());

    for &func_idx in funcs_ids.iter() {
        validate_func(func_idx, &program, &mut call_graph)?;
    }

    call_graph.ensure_no_recursive_calls()?;

    Ok(())
}

fn validate_func(
    func_idx: FuncIndex,
    program: &Program,
    call_graph: &mut CallGraph,
) -> Result<(), ProgramError> {
    let func_body = program.get_func_body(func_idx).to_vec();

    let _ = validate_func_block(func_idx, program, &func_body, 0, call_graph)?;

    Ok(())
}

fn validate_func_block(
    func_idx: FuncIndex,
    program: &Program,
    block_ops: &[Instruction],
    block_offset: usize,
    call_graph: &mut CallGraph,
) -> Result<usize, ProgramError> {
    let mut cursor = block_offset;

    while let Some(op) = block_ops.get(cursor) {
        match *op {
            Instruction::Loop(..) => return Err(ProgramError::LoopNotAllowed),
            Instruction::CallIndirect(..) => return Err(ProgramError::CallIndirectNotAllowed),
            Instruction::Call(to) => {
                validate_func_index(to)?;

                let to = FuncIndex(to as u16);

                if program.is_imported(to) == false {
                    if func_idx == to {
                        return Err(ProgramError::RecursiveCall(vec![func_idx, func_idx]));
                    }

                    call_graph.add_call(func_idx, to);
                }
                cursor += 1;
            }
            Instruction::Block(..) => {
                cursor = validate_func_block(func_idx, program, block_ops, cursor + 1, call_graph)?;
            }
            Instruction::If(..) => {
                let if_cont_cursor =
                    validate_func_block(func_idx, program, block_ops, cursor + 1, call_graph)?;

                if let Some(Instruction::Else) = block_ops.get(if_cont_cursor) {
                    let else_cont_cursor = validate_func_block(
                        func_idx,
                        program,
                        block_ops,
                        if_cont_cursor + 1,
                        call_graph,
                    )?;
                    cursor = else_cont_cursor;
                } else {
                    cursor = if_cont_cursor;
                }
            }
            Instruction::Else => break,
            Instruction::End => {
                cursor += 1;
                break;
            }
            _ => {
                validate_non_float(op)?;

                cursor += 1;
            }
        }
    }

    Ok(cursor)
}

fn validate_func_index(func_idx: u32) -> Result<(), ProgramError> {
    if func_idx <= std::u16::MAX as u32 {
        Ok(())
    } else {
        Err(ProgramError::FunctionIndexTooLarge)
    }
}

#[inline]
fn validate_non_float(op: &Instruction) -> Result<(), ProgramError> {
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
