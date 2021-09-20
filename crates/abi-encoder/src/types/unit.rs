use crate::{traits::Push, ByteSize, Encoder};

impl Encoder for () {
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        w.push(svm_abi_layout::UNIT);
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
