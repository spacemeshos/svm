extern crate alloc;

use alloc::vec::Vec;

use svm_abi_layout::layout;

use crate::Encoder;

impl Encoder for () {
    fn encode(&self, w: &mut Vec<u8>) {
        w.push(layout::UNIT);
    }
}
