use std::fmt::{self, Debug};

use crate::cfg::UnresolvedJump;

use super::BlockNum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cont {
    pub origin: BlockNum,

    pub target: BlockNum,

    pub kind: ContKind,
}

impl Cont {
    pub fn origin(&self) -> BlockNum {
        self.origin
    }

    pub fn target(&self) -> BlockNum {
        self.target
    }

    pub fn kind(&self) -> ContKind {
        self.kind
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ContKind {
    OnIfTrue,
    OnIfFalse,
    AfterThen,
    AfterElse,
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
