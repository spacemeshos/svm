use super::Block;

use std::fmt::{self, Display};

/// `BlockNum` is the `label` type used for each node in a function `CFG`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BlockNum(pub usize);

impl BlockNum {
    /// Increment the inner block number.
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

impl Display for BlockNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_inc() {
        let mut b1 = BlockNum(10);

        b1.inc();
        assert_eq!(b1, BlockNum(11));

        b1.inc();
        assert_eq!(b1, BlockNum(12));
    }
}
