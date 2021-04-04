use crate::{ByteSize, Encoder};

use svm_abi_layout::layout;

macro_rules! encode {
    ($ty:ty, $W:ty, $MARK_1B:expr, $MARK_2B:expr, $MARK_3B:expr, $MARK_4B:expr) => {
        impl Encoder<$W> for $ty {
            fn encode(&self, w: &mut $W) {
                let v = *self as u32;
                let size = self.byte_size();

                match size {
                    2 => {
                        w.push($MARK_1B);
                        w.push(v as u8);
                    }
                    3 => {
                        w.push($MARK_2B);

                        let bytes: [u8; 4] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);

                        w.push(bytes[2]);
                        w.push(bytes[3]);
                    }
                    4 => {
                        w.push($MARK_3B);

                        let bytes: [u8; 4] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                    }
                    5 => {
                        w.push($MARK_4B);

                        let bytes: [u8; 4] = self.to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                    }
                    _ => svm_sdk_std::panic(),
                }
            }
        }
    };
}

encode!(
    i32,
    svm_sdk_std::Vec<u8>,
    layout::I32_1B,
    layout::I32_2B,
    layout::I32_3B,
    layout::I32_4B
);

encode!(
    u32,
    svm_sdk_std::Vec<u8>,
    layout::U32_1B,
    layout::U32_2B,
    layout::U32_3B,
    layout::U32_4B
);

macro_rules! encode_byte_size {
    ($ty:ty) => {
        impl ByteSize for $ty {
            #[inline]
            fn byte_size(&self) -> usize {
                let v = *self as u32;

                match v {
                    0x00..=0xFF => 2,
                    0x01_00..=0xFF_FF => 3,
                    0x_01_00_00..=0xFF_FF_FF => 4,
                    0x01_00_00_00..=0xFF_FF_FF_FF => 5,
                }
            }

            fn max_byte_size() -> usize {
                5
            }
        }
    };
}

encode_byte_size!(i32);
encode_byte_size!(u32);
