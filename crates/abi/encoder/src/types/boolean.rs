use svm_abi_layout::layout;

use crate::{ByteSize, Encoder};

macro_rules! impl_bool {
    ($W:ty) => {
        impl Encoder<$W> for bool {
            fn encode(&self, w: &mut $W) {
                if *self {
                    w.push(layout::BOOL_TRUE);
                } else {
                    w.push(layout::BOOL_FALSE);
                }
            }
        }
    };
}

impl_bool!(svm_sdk_std::Vec<u8>);

impl ByteSize for bool {
    fn byte_size(&self) -> usize {
        1
    }

    fn max_byte_size() -> usize {
        1
    }
}
