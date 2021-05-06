use super::Block;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockNum(pub usize);

impl BlockNum {
    pub fn inc(&mut self) {
        self.0 += 1;
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
