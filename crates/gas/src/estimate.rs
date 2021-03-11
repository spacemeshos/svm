use crate::{
    block::{BlockCtx, FuncsBlocks, OpsBlock},
    call_graph::CallGraph,
    error::ProgramError,
    function::{FuncIndex, FuncsGas},
    gas::Gas,
    op::Op,
    program::Program,
    traits::VMCallsGasEstimator,
};

use std::collections::HashMap;

use parity_wasm::elements::Instruction;

static FUNC_BLOCK_MAX_DEPTH: usize = 256;

/// Recursives a parsed program as `Program`.
/// On success, returns for each function-index its estimated gas.
/// On failure, returns an error.
pub fn estimate_code<VME>(wasm: &[u8]) -> Result<HashMap<FuncIndex, Gas>, ProgramError>
where
    VME: VMCallsGasEstimator,
{
    let program = crate::code_reader::read_program(wasm)?;
    let funcs_ids = program.functions_ids();

    let mut funcs_blocks = FuncsBlocks::new();
    let mut funcs_gas = FuncsGas::new();
    let mut call_graph = CallGraph::new(funcs_ids.clone());

    for &func_idx in funcs_ids.iter() {
        estimate_func(func_idx, &program, &mut funcs_blocks, &mut call_graph)?;
    }

    call_graph.ensure_no_recursive_calls()?;

    for func_idx in call_graph.topological_sort().iter() {
        let gas = estimate_func_gas::<VME>(*func_idx, &funcs_blocks, &funcs_gas);
        funcs_gas.set_func_gas(*func_idx, gas);
    }

    Ok(funcs_gas.inner)
}

fn estimate_func(
    func_idx: FuncIndex,
    program: &Program,
    funcs_blocks: &mut FuncsBlocks,
    call_graph: &mut CallGraph,
) -> Result<(), ProgramError> {
    let func_body = program.get_function_body(func_idx).to_vec();

    let (_, block) = estimate_func_block(func_idx, program, &func_body, 0, call_graph)?;
    funcs_blocks.add_func_block(func_idx, block);

    Ok(())
}

fn estimate_func_block(
    func_idx: FuncIndex,
    program: &Program,
    block_ops: &[Instruction],
    block_offset: usize,
    call_graph: &mut CallGraph,
) -> Result<(usize, OpsBlock), ProgramError> {
    let mut block = OpsBlock::new();
    let mut cursor = block_offset;

    while let Some(op) = block_ops.get(cursor) {
        match *op {
            Instruction::Loop(..) => return Err(ProgramError::LoopNotAllowed),
            Instruction::Br(..) => return Err(ProgramError::BrNotAllowed),
            Instruction::BrIf(..) => return Err(ProgramError::BrIfNotAllowed),
            Instruction::BrTable(..) => return Err(ProgramError::BrTableNotAllowed),
            Instruction::CallIndirect(..) => return Err(ProgramError::CallIndirectNotAllowed),
            Instruction::Call(to) => {
                let to = FuncIndex(to as u16);

                if program.is_imported(to) {
                    block.append(Op::VMCall(to));
                } else {
                    if func_idx == to {
                        return Err(ProgramError::RecursiveCall(vec![func_idx, func_idx]));
                    }

                    call_graph.add_call(func_idx, to);
                    block.append(Op::FuncCall(to));
                }
                cursor += 1;
            }
            Instruction::Block(..) => {
                let (cont_cursor, inner) =
                    estimate_func_block(func_idx, program, block_ops, cursor + 1, call_graph)?;

                block.append(Op::Block(inner));
                cursor = cont_cursor;
            }
            Instruction::If(..) => {
                let (if_cont_cursor, if_block) =
                    estimate_func_block(func_idx, program, block_ops, cursor + 1, call_graph)?;

                if let Some(Instruction::Else) = block_ops.get(if_cont_cursor) {
                    let (else_cont_cursor, else_block) = estimate_func_block(
                        func_idx,
                        program,
                        block_ops,
                        if_cont_cursor + 1,
                        call_graph,
                    )?;

                    block.append(Op::IfElseBlock(if_block, else_block));
                    cursor = else_cont_cursor;
                } else {
                    block.append(Op::IfBlock(if_block));
                    cursor = if_cont_cursor;
                }
            }
            Instruction::Else => break,
            Instruction::End => {
                cursor += 1;
                break;
            }
            _ => {
                block.append(Op::Plain(op.clone()));
                cursor += 1;
            }
        }
    }

    Ok((cursor, block))
}

fn estimate_func_gas<VME>(
    func_idx: FuncIndex,
    funcs_blocks: &FuncsBlocks,
    funcs_gas: &FuncsGas,
) -> Gas
where
    VME: VMCallsGasEstimator,
{
    if let Some(cached) = funcs_gas.get_func_gas(func_idx) {
        return cached;
    }

    let func_block = funcs_blocks.get_func_block(func_idx);
    let block_ctx = BlockCtx::new(func_idx, func_block);

    estimate_block_gas::<VME>(&block_ctx, funcs_gas)
}

fn estimate_block_gas<VME>(ctx: &BlockCtx, funcs_gas: &FuncsGas) -> Gas
where
    VME: VMCallsGasEstimator,
{
    if ctx.depth > FUNC_BLOCK_MAX_DEPTH {
        panic!("function `{}` block depth is too deep", ctx.func_idx.0);
    }

    let mut gas = Gas::Fixed(0);

    for op in ctx.ops.0.iter() {
        let op_gas = match *op {
            Op::Plain(Instruction::Nop) => Gas::Fixed(0),
            Op::Plain(..) => Gas::Fixed(1),
            Op::Block(ref inner) => estimate_block_gas::<VME>(&ctx.child_block(inner), funcs_gas),
            Op::VMCall(fid) => VME::estimate_code(fid),
            Op::FuncCall(fid) => funcs_gas.get_func_gas(fid).unwrap(),
            Op::IfBlock(ref true_block) => {
                let true_gas = estimate_block_gas::<VME>(&ctx.child_block(true_block), funcs_gas);
                let else_gas = Gas::Fixed(0);
                true_gas + else_gas
            }
            Op::IfElseBlock(ref true_block, ref else_block) => {
                let true_gas = estimate_block_gas::<VME>(&ctx.child_block(true_block), funcs_gas);
                let else_gas = estimate_block_gas::<VME>(&ctx.child_block(else_block), funcs_gas);

                true_gas + else_gas
            }
        };

        gas *= op_gas;
    }

    gas
}
