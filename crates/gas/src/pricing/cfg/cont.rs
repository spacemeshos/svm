use super::BlockNum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cont {
    pub origin: BlockNum,

    pub target: BlockNum,
}

impl Cont {
    pub fn origin(&self) -> BlockNum {
        self.origin
    }

    pub fn target(&self) -> BlockNum {
        self.target
    }
}
