use std::fmt::{self, Debug};

mod wasm;
pub use wasm::{WasmBrTable, WasmJump};

mod unresolved;
pub use unresolved::UnresolvedJump;

use super::{BlockNum, Depth};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Jump {
    pub origin: BlockNum,

    pub target: BlockNum,
}

impl Jump {
    pub fn origin(&self) -> BlockNum {
        self.origin
    }

    pub fn target(&self) -> BlockNum {
        self.target
    }
}
