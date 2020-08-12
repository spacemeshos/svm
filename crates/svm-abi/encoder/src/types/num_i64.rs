use svm_abi_layout::layout;
use svm_nibble::NibbleWriter;

use crate::Encoder;

macro_rules! encode {
    ($ty:ty, $MARK_1B:expr, $MARK_2B:expr, $MARK_3B:expr, $MARK_4B:expr, $MARK_5B:expr, $MARK_6B:expr, $MARK_7B:expr, $MARK_8B:expr) => {
        impl Encoder for $ty {
            fn encode(&self, w: &mut NibbleWriter) {
                let v = *self as u64;

                match v {
                    0x00..=0xFF => {
                        w.write_byte($MARK_1B);
                        w.write_byte(v as u8);
                    }
                    0x01_00..=0xFF_FF => {
                        w.write_byte($MARK_2B);

                        let bytes: [u8; 2] = (v as u16).to_be_bytes();
                        w.write_bytes(&bytes);
                    }
                    0x_01_00_00..=0xFF_FF_FF => {
                        w.write_byte($MARK_3B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.write_bytes(&bytes[1..]);
                    }
                    0x_01_00_00_00..=0xFF_FF_FF_FF => {
                        w.write_byte($MARK_4B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();
                        w.write_bytes(&bytes);
                    }
                    0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => {
                        w.write_byte($MARK_5B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);
                        debug_assert_eq!(bytes[2], 0);

                        w.write_bytes(&bytes[3..]);
                    }
                    0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => {
                        w.write_byte($MARK_6B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);

                        w.write_bytes(&bytes[2..]);
                    }
                    0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => {
                        w.write_byte($MARK_7B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.write_bytes(&bytes[1..]);
                    }
                    0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => {
                        w.write_byte($MARK_8B);

                        let bytes: [u8; 8] = v.to_be_bytes();
                        w.write_bytes(&bytes);
                    }
                }
            }
        }
    };
}

encode!(
    i64,
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
    layout::U64_1B,
    layout::U64_2B,
    layout::U64_3B,
    layout::U64_4B,
    layout::U64_5B,
    layout::U64_6B,
    layout::U64_7B,
    layout::U64_8B
);
