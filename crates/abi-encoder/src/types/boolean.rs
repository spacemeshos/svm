use svm_abi_layout::layout;

use crate::{traits::Push, ByteSize, Encoder};

impl Encoder for bool {
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        w.push(if *self {
            layout::BOOL_TRUE
        } else {
            layout::BOOL_FALSE
        });
    }
}

impl ByteSize for bool {
    fn byte_size(&self) -> usize {
        1
    }

    fn max_byte_size() -> usize {
        1
    }
}
