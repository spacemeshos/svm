use seq_macro::seq;
use svm_sdk_types::Amount;

use crate::{traits::Push, ByteSize, Encoder};

impl Encoder for Amount {
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        let size = self.byte_size();

        w.push(layout_amount_b(size as u8 - 2));
        let bytes: [u8; 8] = self.0.to_be_bytes();

        seq!(I in 0..8 {
            if size >= 9 - I {
                w.push(bytes[I]);
            }
        });
    }
}

impl ByteSize for Amount {
    #[inline]
    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }

    fn max_byte_size() -> usize {
        u64::MAX.byte_size()
    }
}

#[inline]
const fn layout_amount_b(i: u8) -> u8 {
    (i << 4) | 1
}
