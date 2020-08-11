extern crate alloc;

use alloc::vec::Vec;

use crate::Encoder;

use svm_nibble::Nibble;

impl Encoder for i8 {
    fn encode(&self, buf: &mut Vec<u8>) {
        //
    }
}
