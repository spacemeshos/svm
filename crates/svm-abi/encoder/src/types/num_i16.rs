use svm_nibble::NibbleWriter;

use crate::{layout, Encoder};

macro_rules! encode {
    ($ty:ty, $MARK_1B:expr, $MARK_2B:expr) => {
        impl Encoder for $ty {
            fn encode(&self, w: &mut NibbleWriter) {
                let v = *self as u16;

                match v {
                    0..=0xFF => {
                        w.write_byte($MARK_1B);
                        w.write_byte(v as u8);
                    }
                    _ => {
                        w.write_byte($MARK_2B);

                        let bytes: [u8; 2] = v.to_be_bytes();
                        w.write_bytes(&bytes)
                    }
                };
            }
        }
    };
}

encode!(i16, layout::I16_1B, layout::I16_2B);
encode!(u16, layout::U16_1B, layout::U16_2B);
