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

    pub func: FuncIndex,

    pub depth: usize,
}

impl<'ctx> BlockContext<'ctx> {
    pub fn new(func: FuncIndex, ops: &'ctx Block) -> Self {
        Self {
            ops,
            func,
            depth: 1,
        }
    }

    pub fn child_block(&self, ops: &'ctx Block) -> Self {
        Self {
            ops,
            func: self.func,
            depth: self.depth + 1,
        }
    }
}
