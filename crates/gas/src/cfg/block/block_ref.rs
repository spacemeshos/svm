use std::fmt::{self, Debug};

use super::{Block, BlockBuilder, BlockNum, Edge, Op};

pub enum BlockRef<'f> {
    NotReady(BlockBuilder<'f>),

    Ready(Block<'f>),

    Empty,
}

impl<'f> BlockRef<'f> {
    pub fn block_num(&self) -> BlockNum {
        match self {
            BlockRef::NotReady(ref builder) => builder.block_num(),
            BlockRef::Ready(ref block) => block.num(),
            BlockRef::Empty => unreachable!(),
        }
    }

    pub fn add_incoming_edge(&mut self, edge: Edge) {
        let builder = self.as_builder_mut();

        builder.add_incoming_edge(edge);
    }

    pub fn add_outgoing_edge(&mut self, edge: Edge) {
        let builder = self.as_builder_mut();

        builder.add_outgoing_edge(edge);
    }

    pub fn append(&mut self, op: Op<'f>) {
        let builder = self.as_builder_mut();

        builder.append(op);
    }

    pub fn replace<'g: 'f>(&mut self, block_ref: BlockRef<'g>) -> BlockRef<'f> {
        std::mem::replace(self, block_ref)
    }

    fn as_builder_mut(&mut self) -> &mut BlockBuilder<'f> {
        match *self {
            BlockRef::NotReady(ref mut builder) => builder,
            _ => unreachable!(
                "expected `BlockBuilder` for `block_num = #{:?}`",
                self.block_num()
            ),
        }
    }

    fn as_ready(&self) -> &Block {
        match *self {
            BlockRef::Ready(ref block) => block,
            _ => unreachable!(
                "expected a ready Block for `block_num = {:?}`",
                self.block_num()
            ),
        }
    }
}

impl<'f> Debug for BlockRef<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockRef::Ready(block) => write!(f, "BlockRef::Block({:?})", block.num()),
            BlockRef::NotReady(builder) => {
                write!(f, "BlockRef::Builder({:?})", builder.block_num())
            }
            BlockRef::Empty => write!(f, "BlockRef::Empty"),
        }
    }
}
