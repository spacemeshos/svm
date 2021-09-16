use svm_abi_layout::layout;

use crate::{traits::Push, ABIEncoder, ByteSize};

impl ABIEncoder for () {
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        w.push(layout::UNIT);
    }
}

impl ByteSize for () {
    fn max_byte_size() -> usize {
        1
    }

    fn byte_size(&self) -> usize {
        1
    }
}
