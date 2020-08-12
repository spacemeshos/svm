extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;

use crate::Encoder;

macro_rules! encode {
    ($ty:ty, $MARK_1B:expr, $MARK_2B:expr) => {
        impl Encoder for $ty {
            fn encode(&self, w: &mut Vec<u8>) {
                let v = *self as u16;

                match v {
                    0..=0xFF => {
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

encode!(i16, layout::I16_1B, layout::I16_2B);
encode!(u16, layout::U16_1B, layout::U16_2B);
