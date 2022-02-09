use super::{BlockNum, Jump};

/// Represents an unresolved `Jump`.
///
/// A `Jump` is considered unresolved when only its `origin` is known (but its `target` isn't)
#[derive(Debug, Clone)]
pub struct UnresolvedJump {
    origin: BlockNum,
}

impl UnresolvedJump {
    /// Creates a new unresolved-`Jump`
    pub fn new(origin: BlockNum) -> Self {
        Self { origin }
    }

    /// Creates the `origin` of an unresolved-`Jump`
    pub fn origin(&self) -> BlockNum {
        self.origin
    }

    /// Resolves the unresolved-`Jump` by supplying its `target` and returns a `Jump`
    pub fn resolve(self, target: BlockNum) -> Jump {
        Jump {
            origin: self.origin,
            target,
        }
    }
}
