use svm_abi_layout::layout;

use crate::{traits::Push, ByteSize, Encoder};

impl<W> Encoder<W> for ()
where
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        w.push(layout::UNIT);
    }
}

impl ByteSize for () {
    fn byte_size(&self) -> usize {
        1
    }

    fn max_byte_size() -> usize {
        1
    }
}
