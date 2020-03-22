use crate::error::Error;
use crate::function::FuncIndex;
use crate::gas::Gas;
use crate::program::Program;
use crate::traits::VMCallsGasEstimator;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use parity_wasm::elements::Instruction;

static FUNC_BLOCK_MAX_DEPTH: usize = 256;

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

struct BlockCtx<'ctx> {
    ops: &'ctx OpsBlock,
    func_idx: FuncIndex,
    depth: usize,
}

impl<'ctx> BlockCtx<'ctx> {
    fn new(func_idx: FuncIndex, ops: &'ctx OpsBlock) -> Self {
        Self {
            ops,
            func_idx,
            depth: 1,
        }
    }

    fn child_block(&self, ops: &'ctx OpsBlock) -> Self {
        Self {
            ops,
            func_idx: self.func_idx,
            depth: self.depth + 1,
        }
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
        self.inner.insert(func_idx, block);
    }

    fn get_func_block(&self, func_idx: FuncIndex) -> &OpsBlock {
        self.inner.get(&func_idx).unwrap()
    }
}

#[derive(Debug)]
pub struct FuncsGas {
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

    fn get_func_gas(&self, func_idx: FuncIndex) -> Option<Gas> {
        match self.inner.get(&func_idx) {
            None => None,
            Some(gas) => Some(*gas),
        }
    }
}

#[derive(Debug)]
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

        self.root_funcs.remove(&from);

        let entry = self.out_calls.entry(from).or_insert_with(HashSet::new);
        entry.insert(to);

        let entry = self.in_calls.entry(to).or_insert_with(HashSet::new);
        entry.insert(from);
    }

    fn ensure_no_recursive_calls(&self) -> Result<(), Error> {
        let mut visited = HashSet::new();

        let mut all_funcs = self.all_funcs.iter().copied().collect::<Vec<FuncIndex>>();

        // we sort `all_funcs` in order to make the unit-tests execution determinstic
        all_funcs.sort();

        for func_idx in all_funcs.iter() {
            let mut path = Vec::new();
            self.internal_graph_traverse(*func_idx, &mut visited, &mut path)?;
        }

        Ok(())
    }

    fn topological_sort(&self) -> Vec<FuncIndex> {
        let mut res = Vec::new();
        let mut out_calls = self.out_calls.clone();

        let mut roots_funcs = self.root_funcs.iter().copied().collect::<Vec<FuncIndex>>();

        while let Some(root) = roots_funcs.pop() {
            res.push(root);

            if let Some(callers) = self.in_calls.get(&root) {
                for caller in callers.iter() {
                    let caller_callees = out_calls.get_mut(&caller).unwrap();

                    assert!(caller_callees.contains(&root));

                    caller_callees.remove(&root);

                    if caller_callees.is_empty() {
                        roots_funcs.push(*caller);
                    }
                }
            }
        }

        assert_eq!(self.all_funcs.len(), res.len());

        res
    }

    fn internal_graph_traverse(
        &self,
        caller: FuncIndex,
        visited: &mut HashSet<FuncIndex>,
        path: &mut Vec<FuncIndex>,
    ) -> Result<(), Error> {
        if visited.contains(&caller) {
            return Ok(());
        }

        if path.contains(&caller) {
            path.push(caller);
            return Err(Error::RecursiveCall(path.clone()));
        }

        path.push(caller);

        if let Some(callees) = self.out_calls.get(&caller) {
            for callee in callees.iter() {
                self.internal_graph_traverse(*callee, visited, path)?;
            }
        }

        visited.insert(caller);

        Ok(())
    }
}

/// Recursives a parsed program as `Program`.
/// On success, returns for each function-index its estimated gas.
/// On failure, returns an error.
pub fn estimate_program<VME>(program: &Program) -> Result<HashMap<FuncIndex, Gas>, Error>
where
    VME: VMCallsGasEstimator,
{
    let funcs_ids = program.functions_ids();

    let mut funcs_blocks = FuncsBlocks::new();
    let mut funcs_gas = FuncsGas::new();
    let mut call_graph = CallGraph::new(funcs_ids.clone());

    for func_idx in funcs_ids.iter() {
        construct_func_block(*func_idx, program, &mut funcs_blocks, &mut call_graph)?;
    }

    call_graph.ensure_no_recursive_calls()?;

    for func_idx in call_graph.topological_sort().iter() {
        let gas = estimate_func_gas::<VME>(*func_idx, &funcs_blocks, &funcs_gas);
        funcs_gas.set_func_gas(*func_idx, gas);
    }

    Ok(funcs_gas.inner)
}

fn construct_func_block(
    func_idx: FuncIndex,
    program: &Program,
    funcs_blocks: &mut FuncsBlocks,
    call_graph: &mut CallGraph,
) -> Result<(), Error> {
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

                if program.is_imported(to) {
                    block.append(Op::VMCall(to));
                } else {
                    if func_idx == to {
                        return Err(Error::RecursiveCall(vec![func_idx, func_idx]));
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
            Op::VMCall(fid) => VME::estimate_gas(fid),
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
