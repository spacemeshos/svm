use svm_abi_layout::layout;

use crate::{ByteSize, Encoder};

macro_rules! encode {
    ($ty:ty, $W:ty, $MARK_1B:expr, $MARK_2B:expr) => {
        impl Encoder<$W> for $ty {
            fn encode(&self, w: &mut $W) {
                let v = *self as u16;

                let size = self.byte_size();

                match size {
                    2 => {
                        w.push($MARK_1B);
                        w.push(v as u8);
                    }
                    _ => {
                        w.push($MARK_2B);

                        let bytes: [u8; 2] = v.to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                    }
                };
            }
        }
    };
}

encode!(i16, svm_sdk_std::Vec<u8>, layout::I16_1B, layout::I16_2B);
encode!(u16, svm_sdk_std::Vec<u8>, layout::U16_1B, layout::U16_2B);

macro_rules! encode_byte_size {
    ($ty:ty) => {
        impl ByteSize for $ty {
            #[inline]
            fn byte_size(&self) -> usize {
                let v = *self as u16;

                match v {
                    0..=0xFF => 2,
                    _ => 3,
                }
            }

            fn max_byte_size() -> usize {
                3
            }
        }
    };
}

encode_byte_size!(i16);
encode_byte_size!(u16);
