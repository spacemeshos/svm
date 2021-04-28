use svm_abi_layout::layout;

use crate::{ByteSize, Encoder};

macro_rules! encode {
    ($ty:ty, $W:ty, $MARK_1B:expr, $MARK_2B:expr, $MARK_3B:expr, $MARK_4B:expr, $MARK_5B:expr, $MARK_6B:expr, $MARK_7B:expr, $MARK_8B:expr) => {
        impl Encoder<$W> for $ty {
            fn encode(&self, w: &mut $W) {
                let v = *self as u64;
                let size = self.byte_size();

                // TODO:
                // for a detailed explanation on how to make the following code
                // more ergonomic see look at `address.rs` under this module.
                // There is also an issue for that: [Issue #230](https://github.com/spacemeshos/svm/issues/230)

                match size {
                    2 => {
                        w.push($MARK_1B);
                        w.push(v as u8);
                    }
                    3 => {
                        w.push($MARK_2B);

                        let bytes: [u8; 2] = (v as u16).to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                    }
                    4 => {
                        w.push($MARK_3B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                    }
                    5 => {
                        w.push($MARK_4B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                    }
                    6 => {
                        w.push($MARK_5B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);
                        debug_assert_eq!(bytes[2], 0);

                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    7 => {
                        w.push($MARK_6B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);

                        w.push(bytes[2]);
                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    8 => {
                        w.push($MARK_7B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    9 => {
                        w.push($MARK_8B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    _ => svm_sdk_std::panic(),
                }
            }
        }
    };
}

encode!(
    i64,
    svm_sdk_std::Vec<u8>,
    layout::I64_1B,
    layout::I64_2B,
    layout::I64_3B,
    layout::I64_4B,
    layout::I64_5B,
    layout::I64_6B,
    layout::I64_7B,
    layout::I64_8B
);

encode!(
    u64,
    svm_sdk_std::Vec<u8>,
    layout::U64_1B,
    layout::U64_2B,
    layout::U64_3B,
    layout::U64_4B,
    layout::U64_5B,
    layout::U64_6B,
    layout::U64_7B,
    layout::U64_8B
);

macro_rules! encode_byte_size {
    ($ty:ty) => {
        impl ByteSize for $ty {
            #[inline]
            fn byte_size(&self) -> usize {
                let v = *self as u64;

                match v {
                    0x00..=0xFF => 2,
                    0x01_00..=0xFF_FF => 3,
                    0x_01_00_00..=0xFF_FF_FF => 4,
                    0x_01_00_00_00..=0xFF_FF_FF_FF => 5,
                    0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => 6,
                    0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => 7,
                    0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => 8,
                    0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => 9,
                }
            }

            fn max_byte_size() -> usize {
                9
            }
        }
    };
}

encode_byte_size!(i64);
encode_byte_size!(u64);
