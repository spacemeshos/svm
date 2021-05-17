use indexmap::IndexSet;

use crate::cfg::Op;

use super::{Block, BlockNum, Depth, Edge};

pub struct BlockBuilder<'f> {
    block_num: BlockNum,

    ops: Vec<Op<'f>>,

    incoming_edges: Vec<Edge>,

    outgoing_edges: Vec<Edge>,
}

impl<'f> BlockBuilder<'f> {
    pub fn new(block_num: BlockNum) -> Self {
        Self {
            block_num,
            ops: Vec::new(),
            incoming_edges: Vec::new(),
            outgoing_edges: Vec::new(),
        }
    }

    pub fn block_num(&self) -> BlockNum {
        self.block_num
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    pub fn append(&mut self, op: Op<'f>) {
        self.ops.push(op);
    }

    pub fn add_incoming_edge(&mut self, edge: Edge) {
        self.incoming_edges.push(edge);
    }

    pub fn add_outgoing_edge(&mut self, edge: Edge) {
        self.outgoing_edges.push(edge);
    }

    pub fn build(mut self) -> Block<'f> {
        let mut incoming_edges = IndexSet::new();
        let mut outgoing_edges = IndexSet::new();

        for e in self.incoming_edges.drain(..) {
            incoming_edges.insert(e);
        }

        for e in self.outgoing_edges.drain(..) {
            outgoing_edges.insert(e);
        }

        Block {
            num: self.block_num,
            ops: self.ops,
            incoming_edges,
            outgoing_edges,
        }
    }
}
