use super::super::Jump;

use super::{BlockNum, Cont};

/// An `Edge` represent a connection (an edge) between `Block`s in the `CFG`.
///
/// Each edge connects two nodes, called `origin` and `target`:
///
/// * `origin` - The source `Block`
/// * `target` - The destination `Block`
///
/// There are two kinds of `Edge`s: `Continuation` and `Jump`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Edge {
    /// An `Edge` representing a `Jump`
    Jump(Jump),

    /// An `Edge` representing a `Continuation`
    Cont(Cont),
}

impl Edge {
    /// Returns the `origin` (i.e source) `Block` of the edge by returning its `BlockNum`
    pub fn origin(&self) -> BlockNum {
        match self {
            Edge::Jump(jump) => jump.origin(),
            Edge::Cont(cont) => cont.origin(),
        }
    }

    /// Returns the `target` (i.e destination) `Block` of the edge by returning its `BlockNum`
    pub fn target(&self) -> BlockNum {
        match self {
            Edge::Jump(jump) => jump.target(),
            Edge::Cont(cont) => cont.target(),
        }
    }
}
