use svm_abi_layout::layout;
use svm_nibble::{nib, NibbleWriter};

use crate::Encoder;

impl Encoder for bool {
    fn encode(&self, w: &mut NibbleWriter) {
        if *self {
            w.push(nib!(layout::BOOL_TRUE));
        } else {
            w.push(nib!(layout::BOOL_FALSE));
        }
    }
}
