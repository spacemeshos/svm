use std::collections::HashMap;

use crate::{FuncIndex, Op};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block(pub Vec<Op>);

impl Block {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(&mut self, op: Op) {
        self.0.push(op);
    }
}

pub(crate) struct BlockContext<'ctx> {
    pub ops: &'ctx Block,

    pub func_idx: FuncIndex,

    pub depth: usize,
}

impl<'ctx> BlockContext<'ctx> {
    pub fn new(func_idx: FuncIndex, ops: &'ctx Block) -> Self {
        Self {
            ops,
            func_idx,
            depth: 1,
        }
    }

    pub fn child_block(&self, ops: &'ctx Block) -> Self {
        Self {
            ops,
            func_idx: self.func_idx,
            depth: self.depth + 1,
        }
    }
}

pub(crate) struct FuncsBlocks {
    inner: HashMap<FuncIndex, Block>,
}

impl FuncsBlocks {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add_func_block(&mut self, func_idx: FuncIndex, block: Block) {
        self.inner.insert(func_idx, block);
    }

    pub fn get_func_block(&self, func_idx: FuncIndex) -> &Block {
        self.inner.get(&func_idx).unwrap()
    }
}
