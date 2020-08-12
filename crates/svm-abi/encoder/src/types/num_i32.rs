extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;

use crate::Encoder;

macro_rules! encode {
    ($ty:ty, $MARK_1B:expr, $MARK_2B:expr, $MARK_3B:expr, $MARK_4B:expr) => {
        impl Encoder for $ty {
            fn encode(&self, w: &mut Vec<u8>) {
                let v = *self as u32;

                match v {
                    0x00..=0xFF_FF => {
                        w.push($MARK_1B);
                        w.push(v as u8);
                    }
                    0x01_00_00..=0xFF_FF_FF => {
                        w.push($MARK_2B);

                        let bytes: [u8; 4] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);

                        w.extend_from_slice(&bytes[2..]);
                    }
                    0x_01_00_00..=0xFF_FF_FF => {
                        w.push($MARK_3B);

                        let bytes: [u8; 4] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.extend_from_slice(&bytes[1..]);
                    }
                    0x01_00_00_00..=0xFF_FF_FF_FF => {
                        w.push($MARK_4B);

                        let bytes: [u8; 4] = self.to_be_bytes();
                        w.extend_from_slice(&bytes);
                    }
                }
            }
        }
    };
}

encode!(
    i32,
    layout::I32_1B,
    layout::I32_2B,
    layout::I32_3B,
    layout::I32_4B
);

encode!(
    u32,
    layout::U32_1B,
    layout::U32_2B,
    layout::U32_3B,
    layout::U32_4B
);
