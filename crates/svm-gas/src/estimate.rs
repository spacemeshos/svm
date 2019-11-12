use crate::error::Error;
use crate::function::{FuncBody, FuncIndex};
use crate::gas::Gas;
use crate::program::Program;

use std::collections::HashMap;

use parity_wasm::elements::Instruction;

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Plain(Instruction),
    Block(OpsBlock),
    IfBlock(OpsBlock),
    IfElseBlock(OpsBlock, OpsBlock),
    VMCall(FuncIndex),
    FuncCall(FuncIndex, OpsBlock),
}

#[derive(Debug, Clone, PartialEq)]
struct OpsBlock(Vec<Op>);

impl OpsBlock {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn append(&mut self, op: Op) {
        self.0.push(op);
    }
}

struct FuncsBlocks {
    inner: HashMap<FuncIndex, OpsBlock>,
}

impl FuncsBlocks {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add_func(&mut self, func_idx: FuncIndex, block: OpsBlock) {
        //
    }
}

struct FuncsGas {
    inner: HashMap<FuncIndex, Gas>,
}

impl FuncsGas {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn set_func_gas(&mut self, func_idx: FuncIndex, gas: Gas) {
        self.inner.insert(func_idx, gas);
    }

    fn get_func_gas(&mut self, func_idx: FuncIndex) -> Option<Gas> {
        match self.inner.get(&func_idx) {
            None => None,
            Some(gas) => Some(gas.clone()),
        }
    }
}

pub fn estimate(program: &Program) -> Result<Gas, Error> {
    // we sort `func_ids`, this is important in order to maintain a determinsitic execution of unit-tests
    let mut funcs_ids: Vec<FuncIndex> = program.functions_ids().clone();
    funcs_ids.sort();

    for func_idx in funcs_ids.drain(..) {
        estimate_func_gas(func_idx, program)?;
    }

    panic!()
}

fn estimate_func_gas(func_idx: FuncIndex, program: &Program) -> Result<(), Error> {
    if program.is_imported(func_idx) {
        return estimate_vmcall(func_idx, program);
    }

    let func_body = program.get_function_body(func_idx).to_vec();

    do_func_estimation(func_idx, &func_body)
}

fn estimate_vmcall(_func_idx: FuncIndex, _program: &Program) -> Result<(), Error> {
    Ok(())
}

fn do_func_estimation(func_idx: FuncIndex, func_body: &Vec<Instruction>) -> Result<(), Error> {
    Ok(())
}

fn estimate_block(ops: &Vec<Instruction>, offset: usize) -> Result<(usize, OpsBlock), Error> {
    let mut block = OpsBlock::new();
    let mut cursor = offset;

    while let Some(op) = ops.get(cursor) {
        match *op {
            Instruction::Loop(..) => return Err(Error::LoopNotAllowed),
            Instruction::Br(..) => return Err(Error::BrNotAllowed),
            Instruction::BrIf(..) => return Err(Error::BrIfNotAllowed),
            Instruction::BrTable(..) => return Err(Error::BrTableNotAllowed),
            Instruction::CallIndirect(..) => return Err(Error::CallIndirectNotAllowed),
            Instruction::Call(func_idx) => unimplemented!(),
            Instruction::Block(..) => {
                let (cont_cursor, inner) = estimate_block(ops, cursor)?;

                block.append(Op::Block(inner));
                cursor = cont_cursor;
            }
            Instruction::If(..) => {
                let (if_cont_cursor, if_block) = estimate_block(ops, cursor)?;

                if let Some(Instruction::Else) = ops.get(if_cont_cursor) {
                    let (else_cont_cursor, else_block) = estimate_block(ops, if_cont_cursor)?;
                    block.append(Op::IfElseBlock(if_block, else_block));
                    cursor = else_cont_cursor;
                } else {
                    block.append(Op::IfBlock(if_block));
                    cursor = if_cont_cursor;
                }
            }
            _ => {
                block.append(Op::Plain(op.clone()));
                cursor += 1;
            }
        }
    }

    Ok((cursor, block))
}
