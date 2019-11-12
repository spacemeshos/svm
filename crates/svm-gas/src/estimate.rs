use crate::error::Error;
use crate::function::{FuncBody, FuncIndex};
use crate::gas::Gas;
use crate::program::Program;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use parity_wasm::elements::Instruction;

static FUNC_BLOCK_MAX_DEPTH: usize = 1024;

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Plain(Instruction),
    Block(OpsBlock),
    IfBlock(OpsBlock),
    IfElseBlock(OpsBlock, OpsBlock),
    VMCall(FuncIndex),
    FuncCall(FuncIndex),
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

    fn add_func_block(&mut self, func_idx: FuncIndex, block: OpsBlock) {
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

struct CallGraph {
    all_funcs: HashSet<FuncIndex>,
    root_funcs: HashSet<FuncIndex>,
    in_calls: HashMap<FuncIndex, HashSet<FuncIndex>>,
    out_calls: HashMap<FuncIndex, HashSet<FuncIndex>>,
}

impl CallGraph {
    fn new(funcs_ids: Vec<FuncIndex>) -> Self {
        let all_funcs = HashSet::from_iter(funcs_ids);
        let root_funcs = all_funcs.clone();

        Self {
            all_funcs,
            root_funcs,
            out_calls: HashMap::new(),
            in_calls: HashMap::new(),
        }
    }

    fn add_call(&mut self, from: FuncIndex, to: FuncIndex) {
        assert!(from != to);
        assert!(self.all_funcs.contains(&from));
        assert!(self.all_funcs.contains(&to));

        self.root_funcs.remove(&to);

        let entry = self.out_calls.entry(from).or_insert(HashSet::new());
        entry.insert(to);

        let entry = self.in_calls.entry(to).or_insert(HashSet::new());
        entry.insert(from);
    }

    fn topological_sort(&self) -> Result<Vec<FuncIndex>, Error> {
        let mut res = Vec::new();

        Ok(res)
    }
}

pub fn estimate_program(program: &Program) -> Result<Gas, Error> {
    // we sort `func_ids`, this is important in order to maintain a determinsitic execution of unit-tests
    let mut funcs_ids: Vec<FuncIndex> = program.functions_ids().clone();
    funcs_ids.sort();

    let mut funcs_blocks = FuncsBlocks::new();
    let mut funcs_gas = FuncsGas::new();
    let mut call_graph = CallGraph::new(funcs_ids.clone());

    for func_idx in funcs_ids.iter() {
        construct_func_block(*func_idx, program, &mut funcs_blocks, &mut call_graph)?;
    }

    // for func_idx in call_graph.topological_sort()?.iter() {
    //     estimate_func_gas(*func_idx, &funcs_blocks, &mut funcs_gas);
    // }

    panic!()
}

fn construct_func_block(
    func_idx: FuncIndex,
    program: &Program,
    funcs_blocks: &mut FuncsBlocks,
    call_graph: &mut CallGraph,
) -> Result<(), Error> {
    let func_body = program.get_function_body(func_idx).to_vec();

    do_func_estimation(func_idx, &func_body, funcs_blocks, call_graph)
}

fn append_vmcall(_func_idx: FuncIndex, _program: &Program) -> Result<(), Error> {
    Ok(())
}

fn do_func_estimation(
    func_idx: FuncIndex,
    func_body: &Vec<Instruction>,
    funcs_blocks: &mut FuncsBlocks,
    call_graph: &mut CallGraph,
) -> Result<(), Error> {
    let (_, block) = estimate_func_block(func_idx, func_body, 0, call_graph)?;
    funcs_blocks.add_func_block(func_idx, block);

    Ok(())
}

fn estimate_func_block(
    func_idx: FuncIndex,
    block_ops: &Vec<Instruction>,
    block_offset: usize,
    call_graph: &mut CallGraph,
) -> Result<(usize, OpsBlock), Error> {
    let mut block = OpsBlock::new();
    let mut cursor = block_offset;

    while let Some(op) = block_ops.get(cursor) {
        match *op {
            Instruction::Loop(..) => return Err(Error::LoopNotAllowed),
            Instruction::Br(..) => return Err(Error::BrNotAllowed),
            Instruction::BrIf(..) => return Err(Error::BrIfNotAllowed),
            Instruction::BrTable(..) => return Err(Error::BrTableNotAllowed),
            Instruction::CallIndirect(..) => return Err(Error::CallIndirectNotAllowed),
            Instruction::Call(to) => {
                let to = FuncIndex(to);

                if is_imported_func(to) {
                    block.append(Op::VMCall(to));
                } else {
                    call_graph.add_call(func_idx, to);
                    block.append(Op::FuncCall(to));
                }
                cursor += 1;
            }
            Instruction::Block(..) => {
                let (cont_cursor, inner) =
                    estimate_func_block(func_idx, block_ops, cursor, call_graph)?;

                block.append(Op::Block(inner));
                cursor = cont_cursor;
            }
            Instruction::If(..) => {
                let (if_cont_cursor, if_block) =
                    estimate_func_block(func_idx, block_ops, cursor, call_graph)?;

                if let Some(Instruction::Else) = block_ops.get(if_cont_cursor) {
                    let (else_cont_cursor, else_block) =
                        estimate_func_block(func_idx, block_ops, if_cont_cursor, call_graph)?;

                    block.append(Op::IfElseBlock(if_block, else_block));
                    cursor = else_cont_cursor;
                } else {
                    block.append(Op::IfBlock(if_block));
                    cursor = if_cont_cursor;
                }
            }
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

fn is_imported_func(func_idx: FuncIndex) -> bool {
    false
}
