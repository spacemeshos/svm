use std::collections::HashMap;

use parity_wasm::elements::Instruction;

use crate::read::read_program;
use crate::{Block, BlockContext, CallGraph, FuncIndex, Gas, Op, Program, ProgramError};

mod function;
mod import;
mod instruction;
mod program;

pub use function::FuncPrice;
pub use import::{ImportPriceResolver, Imports};
pub use instruction::InstructionPriceResolver;
pub use program::ProgramResolver;

static FUNC_BLOCK_MAX_DEPTH: usize = 256;

/// Recursively parses a wasm program function-by-function;
///
/// On success, returns for each function-index its gas price (in gas-units).
/// On failure, returns an error.
pub fn price_wasm(
    wasm: &[u8],
    resolver: &impl InstructionPriceResolver,
) -> Result<HashMap<FuncIndex, Gas>, ProgramError> {
    let program = read_program(wasm)?;

    todo!()
    // let mut call_graph = CallGraph::new(functions.clone());

    // for &func in functions.iter() {
    //     price_func(func, &program, &mut funcs_blocks, &mut call_graph)?;
    // }

    // call_graph.assert_no_recursive_calls()?;

    // for func_idx in call_graph.topological_sort().iter() {
    //     let gas = price_func(*func_idx, &funcs_blocks, &funcs_gas, import_resolver);
    //     funcs_gas.set_func_gas(*func_idx, gas);
    // }

    // Ok(funcs_gas.into_inner())
}

// fn price_func<R>(
//     func: FuncIndex,
//     program: &Program,
//     funcs_blocks: &mut FuncsBlocks,
//     call_graph: &mut CallGraph,
//     import_resolver: R,
// ) -> Result<(), ProgramError>
// where
//     R: ImportPriceResolver,
// {
//     let func_body = program.get_func_body(func).instructions();

//     let (_, block) = price_func_block(func, program, &func_body, 0, call_graph)?;
//     funcs_blocks.add_func_block(func, block);

//     Ok(())
// }

// fn price_func_block(
//     func: FuncIndex,
//     program: &Program,
//     block_ops: &[Instruction],
//     block_offset: usize,
//     call_graph: &mut CallGraph,
// ) -> Result<(usize, Block), ProgramError> {
//     let mut block = Block::new();
//     let mut cursor = block_offset;

//     while let Some(op) = block_ops.get(cursor) {
//         match *op {
//             Instruction::Loop(..) => return Err(ProgramError::LoopNotAllowed),
//             Instruction::CallIndirect(..) => return Err(ProgramError::CallIndirectNotAllowed),
//             Instruction::Call(to) => {
//                 let to = FuncIndex(to as u16);

//                 if program.is_imported(to) {
//                     block.append(Op::HostCall(to));
//                 } else {
//                     if func == to {
//                         return Err(ProgramError::CallCycle(vec![func, func]));
//                     }

//                     call_graph.add_call(func, to);
//                     block.append(Op::LocalCall(to));
//                 }
//                 cursor += 1;
//             }
//             Instruction::Block(..) => {
//                 let (cont_cursor, inner) =
//                     price_func_block(func, program, block_ops, cursor + 1, call_graph)?;

//                 block.append(Op::Block(inner));
//                 cursor = cont_cursor;
//             }
//             Instruction::If(..) => {
//                 let (if_cont_cursor, if_block) =
//                     price_func_block(func, program, block_ops, cursor + 1, call_graph)?;

//                 if let Some(Instruction::Else) = block_ops.get(if_cont_cursor) {
//                     let (else_cont_cursor, else_block) =
//                         price_func_block(func, program, block_ops, if_cont_cursor + 1, call_graph)?;

//                     block.append(Op::IfElseBlock(if_block, else_block));
//                     cursor = else_cont_cursor;
//                 } else {
//                     block.append(Op::IfBlock(if_block));
//                     cursor = if_cont_cursor;
//                 }
//             }
//             Instruction::Else => break,
//             Instruction::End => {
//                 cursor += 1;
//                 break;
//             }
//             _ => {
//                 block.append(Op::Instruction(op.clone()));
//                 cursor += 1;
//             }
//         }
//     }

//     Ok((cursor, block))
// }

// fn price_func_gas(
//     func: FuncIndex,
//     funcs_blocks: &FuncsBlocks,
//     funcs_gas: &FuncGas,
//     import_resolver: R,
// ) -> Gas
// where
//     R: ImportPriceResolver,
// {
//     if let Some(gas) = funcs_gas.get_func_gas(func) {
//         return gas;
//     }

//     let func_block = funcs_blocks.get_func_block(func);
//     let block_ctx = BlockContext::new(func, func_block);

//     price_block::<R>(&block_ctx, funcs_gas, import_resolver)
// }

// fn price_block<R>(ctx: &BlockContext, funcs_gas: &FuncGas, import_resolver: R) -> Gas
// where
//     R: ImportPriceResolver,
// {
//     if ctx.depth > FUNC_BLOCK_MAX_DEPTH {
//         panic!("function `{}` block depth is too deep", ctx.func_idx.0);
//     }

//     let mut gas = Gas::Fixed(0);

//     for op in ctx.ops.0.iter() {
//         let op_gas = match *op {
//             Op::Instruction(Instruction::Nop) => Gas::Fixed(0),
//             Op::Instruction(..) => Gas::Fixed(1),
//             Op::Block(ref inner) => import_resolver.price_block(&ctx.child_block(inner), funcs_gas),
//             Op::HostCall(fid) => import_resolver.price_for(fid),
//             Op::LocalCall(fid) => funcs_gas.get_func_gas(fid).unwrap(),
//             Op::IfBlock(ref true_block) => {
//                 let true_gas =
//                     price_block(&ctx.child_block(true_block), funcs_gas, import_resolver);
//                 let else_gas = Gas::Fixed(0);
//                 true_gas + else_gas
//             }
//             Op::IfElseBlock(ref true_block, ref else_block) => {
//                 let true_gas =
//                     price_block(&ctx.child_block(true_block), funcs_gas, import_resolver);
//                 let else_gas =
//                     price_block(&ctx.child_block(else_block), funcs_gas, import_resolver);

//                 true_gas + else_gas
//             }
//         };

//         gas *= op_gas;
//     }

//     gas
// }
