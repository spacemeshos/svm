use indexmap::IndexSet;

use crate::cfg::Op;

use super::{Block, BlockNum, Depth, Edge};

/// Used to build `Block`s
pub struct BlockBuilder<'f> {
    block_num: BlockNum,

    ops: Vec<Op<'f>>,

    incoming_edges: Vec<Edge>,

    outgoing_edges: Vec<Edge>,
}

impl<'f> BlockBuilder<'f> {
    /// New builder
    pub fn new(block_num: BlockNum) -> Self {
        Self {
            block_num,
            ops: Vec::new(),
            incoming_edges: Vec::new(),
            outgoing_edges: Vec::new(),
        }
    }

    /// Returns the `BlockNum` of the currently built `Block`
    pub fn block_num(&self) -> BlockNum {
        self.block_num
    }

    /// Returns whether the currently built `Block` contains code or not
    /// Regardless of whether there are incoming or outgoing edges.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Appends an `Op` to the code of the currently built `Block`
    pub fn append(&mut self, op: Op<'f>) {
        self.ops.push(op);
    }

    /// Adds an incoming `Edge`.
    ///
    /// The `target` of the edge must equal the `BlockNum` of the currently built `Block`.  
    pub fn add_incoming_edge(&mut self, edge: Edge) {
        debug_assert_eq!(edge.target(), self.block_num());

        self.incoming_edges.push(edge);
    }

    /// Adds an outgoing `Edge`.
    ///
    /// The `origin` of the edge must equal the `BlockNum` of the currently built `Block`.  
    pub fn add_outgoing_edge(&mut self, edge: Edge) {
        debug_assert_eq!(edge.origin(), self.block_num());

        self.outgoing_edges.push(edge);
    }

    /// Finalizes the building process. Outputs the newly built `Block`
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
