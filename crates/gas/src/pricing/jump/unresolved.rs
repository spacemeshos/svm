use super::{BlockNum, Depth, Jump};

#[derive(Debug, Clone)]
pub struct UnresolvedJump {
    origin: BlockNum,
}

impl UnresolvedJump {
    pub fn new(origin: BlockNum) -> Self {
        Self { origin }
    }

    pub fn origin(&self) -> BlockNum {
        self.origin
    }

    pub fn resolve(self, target: BlockNum) -> Jump {
        Jump {
            origin: self.origin,
            target,
        }
    }
}
