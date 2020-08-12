extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;
use svm_sdk::Amount;

use crate::Encoder;

macro_rules! encode {
    ($ty:ty, $MARK_1B:expr, $MARK_2B:expr, $MARK_3B:expr, $MARK_4B:expr, $MARK_5B:expr, $MARK_6B:expr, $MARK_7B:expr, $MARK_8B:expr) => {
        impl Encoder for $ty {
            fn encode(&self, w: &mut Vec<u8>) {
                let v = self.0;

                match v {
                    0x00..=0xFF => {
                        w.push($MARK_1B);
                        w.push(v as u8);
                    }
                    0x01_00..=0xFF_FF => {
                        w.push($MARK_2B);

                        let bytes: [u8; 2] = (v as u16).to_be_bytes();
                        w.extend_from_slice(&bytes);
                    }
                    0x_01_00_00..=0xFF_FF_FF => {
                        w.push($MARK_3B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.extend_from_slice(&bytes[1..]);
                    }
                    0x_01_00_00_00..=0xFF_FF_FF_FF => {
                        w.push($MARK_4B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();
                        w.extend_from_slice(&bytes);
                    }
                    0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => {
                        w.push($MARK_5B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);
                        debug_assert_eq!(bytes[2], 0);

                        w.extend_from_slice(&bytes[3..]);
                    }
                    0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => {
                        w.push($MARK_6B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);

                        w.extend_from_slice(&bytes[2..]);
                    }
                    0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => {
                        w.push($MARK_7B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.extend_from_slice(&bytes[1..]);
                    }
                    0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => {
                        w.push($MARK_8B);

                        let bytes: [u8; 8] = v.to_be_bytes();
                        w.extend_from_slice(&bytes);
                    }
                }
            }
        }
    };
}

encode!(
    Amount,
    layout::AMOUNT_1B,
    layout::AMOUNT_2B,
    layout::AMOUNT_3B,
    layout::AMOUNT_4B,
    layout::AMOUNT_5B,
    layout::AMOUNT_6B,
    layout::AMOUNT_7B,
    layout::AMOUNT_8B
);
