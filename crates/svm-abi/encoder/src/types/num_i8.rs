use svm_abi_layout::layout;
use svm_nibble::NibbleWriter;

use crate::Encoder;

impl Encoder for u8 {
    fn encode(&self, w: &mut NibbleWriter) {
        w.write_byte(layout::U8);
        w.write_byte(*self);
    }
}

impl Encoder for i8 {
    #[inline]
    fn encode(&self, w: &mut NibbleWriter) {
        w.write_byte(layout::I8);
        w.write_byte(*self as u8);
    }
}
