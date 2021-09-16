use seq_macro::seq;
use svm_abi_layout::layout;
use svm_sdk_types::Address;

use crate::traits::Push;
use crate::{ABIEncoder, ByteSize};

impl ABIEncoder for Address {
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        w.push(layout::ADDRESS);

        let bytes = self.as_slice();
        seq!(N in 0..20 {
            w.push(bytes[N]);
        });
    }
}

impl ByteSize for Address {
    fn max_byte_size() -> usize {
        21
    }

    fn byte_size(&self) -> usize {
        21
    }
}
