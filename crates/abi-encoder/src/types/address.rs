use seq_macro::seq;
use svm_sdk_types::Address;

use crate::traits::Push;
use crate::{ByteSize, Encoder};

impl Encoder for Address {
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        w.push(svm_byte_layout::ADDRESS);

        let bytes = self.as_slice();
        seq!(N in 0..20 {
            w.push(bytes[N]);
        });
    }
}

impl ByteSize for Address {
    fn byte_size(&self) -> usize {
        21
    }

    fn max_byte_size() -> usize {
        21
    }
}
