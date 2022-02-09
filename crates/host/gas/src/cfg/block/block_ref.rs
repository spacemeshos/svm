use std::fmt::{self, Debug};

use super::{Block, BlockBuilder, BlockNum, Edge, Op};

/// A `BlockRef` is a reference for one of:
///
/// * Not ready yet `Block` (i.e it's still being built)
///
/// * Ready `Block`
///
/// * No Block - references no `Block.
///   Generally, it's a temporary state that should be use when doing: `BlockRef#replace(BlockRef::Empty)`
///   In such a case we are setting an `BlockRef::Empty` to a reference, and get in exchange its previous value.
pub enum BlockRef<'f> {
    /// Referenced `Block` is not ready (still being built)
    NotReady(BlockBuilder<'f>),

    /// Referenced `Block` is ready
    Ready(Block<'f>),

    /// No referenced `Block`. See more explanation above
    Empty,
}

impl<'f> BlockRef<'f> {
    /// Returns the `BlockNum` of the referenced `Block` (whether it's ready yet or not)
    pub fn block_num(&self) -> BlockNum {
        match self {
            BlockRef::NotReady(ref builder) => builder.block_num(),
            BlockRef::Ready(ref block) => block.num(),
            BlockRef::Empty => unreachable!(),
        }
    }

    /// Adds an incoming `Edge` to the referenced `Block` (relevant only for `NotReady` referenced `Block`s)
    pub fn add_incoming_edge(&mut self, edge: Edge) {
        let builder = self.as_builder_mut();

        builder.add_incoming_edge(edge);
    }

    /// Adds an outgoing `Edge` to the referenced `Block` (relevant only for `NotReady` referenced `Block`s)
    pub fn add_outgoing_edge(&mut self, edge: Edge) {
        let builder = self.as_builder_mut();

        builder.add_outgoing_edge(edge);
    }

    /// Appends an `Op` to the referenced `Block` instructions (relevant only for `NotReady` referenced `Block`s)
    pub fn append(&mut self, op: Op<'f>) {
        let builder = self.as_builder_mut();

        builder.append(op);
    }

    /// Replaces the value with the input `BlockRef` and returns the previous value.
    pub fn replace<'g: 'f>(&mut self, block_ref: BlockRef<'g>) -> BlockRef<'f> {
        std::mem::replace(self, block_ref)
    }

    /// Mutually borrows the `BlockBuilder` used when referenced `Block` is in `NotReady` state
    fn as_builder_mut(&mut self) -> &mut BlockBuilder<'f> {
        match *self {
            BlockRef::NotReady(ref mut builder) => builder,
            _ => unreachable!(
                "expected `BlockBuilder` for `block_num = #{:?}`",
                self.block_num()
            ),
        }
    }

    /// Borrows the `Block` used when referenced `Block` is in `Ready` state
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
