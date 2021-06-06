use std::fmt::{self, Debug};

use crate::cfg::UnresolvedJump;

use super::BlockNum;

/// Represents a continuation `Edge`s.
///
/// Each continuation connects between` an `origin` to `target` `Blocks`
/// In addition each continuation has a `ContKind` associated (more explanation under `ContKind` later)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cont {
    /// Continuation's `origin` Block
    pub origin: BlockNum,

    /// Continuation's `target` Block
    pub target: BlockNum,

    /// Continuation's `kind`
    pub kind: ContKind,
}

impl Cont {
    /// Returns the continuation `origin` Block
    pub fn origin(&self) -> BlockNum {
        self.origin
    }

    /// Returns the continuation's `target` Block
    pub fn target(&self) -> BlockNum {
        self.target
    }

    /// Returns the continuation's `kind`
    pub fn kind(&self) -> ContKind {
        self.kind
    }
}

/// Each continuation has a kind associated with.
///
/// Continuations are created in various of cases - we'd like to supplement each one with the reason
/// it was created that's why we've introduced the `kind` field for `Cont`.
///
/// It usage is mainly to assist in debugging/testing created `CFG`s
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ContKind {
    /// `on-if-true` continuation connects an `if-condition` `Block` to the `on-true` `Block`.
    /// The `on-true` Block should be executed when the if-condition is satisfied.
    OnIfTrue,

    /// `on-false-true` continuation connects an `if-condition` `Block` to the `on-false` `Block`.
    /// The `on-false` Block should be executed when the if-condition is NOT satisfied.
    ///
    /// If there is an `else` Block then he `on-false` continuation will link to the start of the `else` `Block`.
    /// When, there is no `else` Block - the `on-false` continuation will point to the same `Block` as the `after-then` one points to.
    OnIfFalse,

    /// The `after-then` continuation connects the last `Block` created when the `if-condition` is met flow
    /// to the first `Block` that succeeds the `after-if` code.
    ///
    /// The `after-then` and `after-else` continuations both target the same `Block`
    /// (both continuations point to the same `target` but have different `origin`)  
    AfterThen,

    /// The `after-then` continuation connects the last `Block` created on the `else` execution flow,
    /// to the first `Block` that succeeds the `after-else` code.
    ///
    /// The `after-then` and `after-else` continuations both target the same `Block`
    /// (both continuations point to the same `target` but have different `origin`)  
    AfterElse,

    /// For `default` continuations. Used mainly when there is a conditional-jump in the code.
    /// In that case, there is a `Block` created for executions when the code skips the jump.
    ///
    /// The `default` continuation links between the preceding `Block` (which is closes since we've reached a jumping instruction)
    /// to the new `when-no-jump` `Block`
    Default,
}

impl Debug for ContKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OnIfTrue => write!(f, "on-if-true"),
            Self::OnIfFalse => write!(f, "on-if-false"),
            Self::AfterThen => write!(f, "after-then"),
            Self::AfterElse => write!(f, "after-else"),
            Self::Default => write!(f, "default"),
        }
    }
}

impl From<&'static str> for ContKind {
    fn from(value: &'static str) -> Self {
        match value {
            "on-if-true" => ContKind::OnIfTrue,
            "on-if-false" => ContKind::OnIfFalse,
            "after-then" => ContKind::AfterThen,
            "after-else" => ContKind::AfterElse,
            "default" => ContKind::Default,
            _ => unreachable!("Invalid `value`={}", value),
        }
    }
}

#[derive(Debug)]
pub struct DepthUnresolvedCont {
    on_if_false: Option<BlockNum>,

    after_if: Option<BlockNum>,

    after_then: Option<BlockNum>,
}

impl DepthUnresolvedCont {
    pub fn is_empty(&self) -> bool {
        self.on_if_false.is_none() && self.after_if.is_none() && self.after_then.is_none()
    }

    pub fn has_on_if_false(&self) -> bool {
        self.on_if_false.is_some()
    }

    pub fn take_on_if_false(&mut self) -> Option<BlockNum> {
        self.on_if_false.take()
    }

    pub fn set_on_if_false(&mut self, on_if_false: BlockNum) {
        self.on_if_false = Some(on_if_false);
    }

    pub fn has_after_if(&self) -> bool {
        self.after_if.is_some()
    }

    pub fn take_after_if(&mut self) -> Option<BlockNum> {
        self.after_if.take()
    }

    pub fn set_after_if(&mut self, after_if: BlockNum) {
        self.after_if = Some(after_if);
    }

    pub fn has_after_then(&self) -> bool {
        self.after_then.is_some()
    }

    pub fn take_after_then(&mut self) -> Option<BlockNum> {
        self.after_then.take()
    }

    pub fn set_after_then(&mut self, after_then: BlockNum) {
        self.after_then = Some(after_then);
    }
}

impl Default for DepthUnresolvedCont {
    fn default() -> Self {
        Self {
            on_if_false: None,
            after_if: None,
            after_then: None,
        }
    }
}
