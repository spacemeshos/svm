use std::fmt::{self, Debug};

mod wasm;
pub use wasm::{WasmBrTable, WasmJump};

mod unresolved;
pub use unresolved::UnresolvedJump;

use super::{BlockNum, Depth};

/// Represents a `Jump` in the code.
///
/// In Wasm there are a couple of branching instructions: `br / br_if / br_table`.
/// A branch instruction can result in `UMP-ing to other locations in the code. Without going here about the nuances of each branching instruction,
/// we want to draw a `Jump`-edges in the CFG between possible jumps. We are able to do that since there is no arbitrary `goto`(s) in Wasm.
///  The control-flow is structured and we can determine the targets of each branch.
///
/// Note: we treat `return` and `unreachable` the same as branch. We look at it as "jumping" out of the function.
/// This plays well with our design, and that's the reason why we reserve `Scope #0` to the function's entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Jump {
    /// The `origin`'s `BlockNum`
    pub origin: BlockNum,

    /// The `target`'s `BlockNum`
    pub target: BlockNum,
}

impl Jump {
    /// Returns the jump's `origin` Block
    pub fn origin(&self) -> BlockNum {
        self.origin
    }

    /// Returns the jump's `target` Block
    pub fn target(&self) -> BlockNum {
        self.target
    }
}
