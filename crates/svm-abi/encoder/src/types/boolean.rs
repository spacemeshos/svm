extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;

use crate::Encoder;

impl Encoder for bool {
    fn encode(&self, w: &mut Vec<u8>) {
        if *self {
            w.push(layout::BOOL_TRUE);
        } else {
            w.push(layout::BOOL_FALSE);
        }
    }
}
