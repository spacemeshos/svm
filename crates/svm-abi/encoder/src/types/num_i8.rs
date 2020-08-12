extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;

use crate::Encoder;

impl Encoder for u8 {
    fn encode(&self, w: &mut Vec<u8>) {
        w.push(layout::U8);
        w.push(*self);
    }
}

impl Encoder for i8 {
    #[inline]
    fn encode(&self, w: &mut Vec<u8>) {
        w.push(layout::I8);
        w.push(*self as u8);
    }
}
