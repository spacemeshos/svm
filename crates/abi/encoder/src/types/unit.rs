use svm_abi_layout::layout;

use crate::{ByteSize, Encoder};

macro_rules! encode {
    ($W:ty) => {
        impl Encoder<$W> for () {
            fn encode(&self, w: &mut $W) {
                w.push(layout::UNIT);
            }
        }
    };
}

encode!(svm_sdk_std::Vec<u8>);

impl ByteSize for () {
    fn byte_size(&self) -> usize {
        1
    }

    fn max_byte_size() -> usize {
        1
    }
}
