use svm_abi_layout::layout;

use crate::{ByteSize, Encoder};

macro_rules! encode {
    ($W:ty) => {
        impl Encoder<$W> for u8 {
            fn encode(&self, w: &mut $W) {
                w.push(layout::U8);
                w.push(*self);
            }
        }

        impl Encoder<$W> for i8 {
            #[inline]
            fn encode(&self, w: &mut $W) {
                w.push(layout::I8);
                w.push(*self as u8);
            }
        }
    };
}

encode!(svm_sdk_std::Vec<u8>);

macro_rules! impl_byte_size {
    ($ty:ty) => {
        impl ByteSize for $ty {
            fn byte_size(&self) -> usize {
                2
            }

            fn max_byte_size() -> usize {
                2
            }
        }
    };
}

impl_byte_size!(i8);
impl_byte_size!(u8);
