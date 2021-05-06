use super::super::Jump;

use super::{BlockNum, Cont};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Edge {
    Jump(Jump),

    Cont(Cont),
}

impl Edge {
    pub fn origin(&self) -> BlockNum {
        match self {
            Edge::Jump(jump) => jump.origin(),
            Edge::Cont(cont) => cont.origin(),
        }
    }

    pub fn target(&self) -> BlockNum {
        match self {
            Edge::Jump(jump) => jump.target(),
            Edge::Cont(cont) => cont.target(),
        }
    }
}
