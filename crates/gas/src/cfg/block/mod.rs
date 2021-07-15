use indexmap::IndexSet;

use svm_program::Op;

use std::fmt::{self, Debug};

use super::{Depth, Edge, Jump};

mod block_num;
mod block_ref;
mod builder;

pub use block_num::BlockNum;
pub use block_ref::BlockRef;
pub use builder::BlockBuilder;

/// A `Block` is the node type used for `CFG`s
#[derive(PartialEq)]
pub struct Block<'f> {
    /// Block's number
    pub num: BlockNum,

    /// Block's code
    pub ops: Vec<Op<'f>>,

    /// Block's incoming `Edge`s
    pub incoming_edges: IndexSet<Edge>,

    /// Block's outgoing `Edge`s
    pub outgoing_edges: IndexSet<Edge>,
}

impl<'f> Block<'f> {
    /// Returns the `Block` id
    pub fn num(&self) -> BlockNum {
        self.num
    }

    /// Returns a slice for the code associated with the `Block`
    pub fn ops(&self) -> &[Op] {
        &self.ops
    }

    /// Returns whether there is code associated with the `Block`.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Returns the first instruction `offset` of the code.
    ///
    /// When there is no code (i.e `Block` is considered empty) - returns `None`
    pub fn start_offset(&self) -> Option<usize> {
        let first = self.ops.first();

        first.map(|op| op.offset())
    }

    /// Returns the last instruction `offset` of the code.
    ///
    /// When there is no code (i.e `Block` is considered empty) - returns `None`
    pub fn end_offset(&self) -> Option<usize> {
        self.start_offset()
            .map(|start_off| start_off + self.ops.len() - 1)
    }

    /// Returns a borrowed Set to the `Block` outgoing edges
    pub fn outgoing_edges(&self) -> &IndexSet<Edge> {
        &self.outgoing_edges
    }

    /// Returns a borrowed Set to the `Block` incoming edges
    pub fn incoming_edges(&self) -> &IndexSet<Edge> {
        &self.incoming_edges
    }
}

impl<'f> Debug for Block<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            writeln!(f, "Block #{} (Empty)", self.num().0)?
        } else {
            writeln!(
                f,
                "Block #{} (offsets: [{:?} - {:?}])",
                self.num().0,
                self.start_offset().unwrap(),
                self.end_offset().unwrap(),
            )?;
        };

        writeln!(f, "\tOutgoing Edges:")?;

        if self.outgoing_edges().is_empty() == false {
            for edge in self.outgoing_edges() {
                self.fmt_edge(f, edge)?;
            }
        } else {
            writeln!(f, "\t\tN/A")?;
        }

        writeln!(f, "\n\tIncoming Edges:")?;

        if self.incoming_edges().is_empty() == false {
            for edge in self.incoming_edges() {
                self.fmt_edge(f, edge)?;
            }
        } else {
            writeln!(f, "\t\tN/A")?;
        }

        writeln!(f, "\n\tInstructions:")?;

        if self.is_empty() == false {
            for op in self.ops.iter() {
                writeln!(f, "\t\t{:?}", op)?;
            }
        } else {
            writeln!(f, "\t\tN/A")?;
        }

        Ok(())
    }
}

impl<'f> Block<'f> {
    fn fmt_edge(&self, f: &mut fmt::Formatter, edge: &Edge) -> fmt::Result {
        let origin = edge.origin();
        let target = edge.target();

        match edge {
            Edge::Jump(..) => writeln!(f, "\t\t{} -> {} (Wasm branch)", origin.0, target.0)?,
            Edge::Cont(cont) => writeln!(
                f,
                "\t\t{} -> {} (`{:?}` Cont.)",
                origin.0,
                target.0,
                cont.kind()
            )?,
        }

        Ok(())
    }
}
