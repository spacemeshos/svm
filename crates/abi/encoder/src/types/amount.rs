use seq_macro::seq;
use svm_sdk_types::Amount;

use crate::{traits::Push, ByteSize, Encoder};

impl<W> Encoder<W> for Amount
where
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        let v = self.0;
        let size = self.byte_size();

        w.push(svm_abi_layout::layout::amount_b(size as u8 - 2));
        let bytes: [u8; 8] = v.to_be_bytes();

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
        let v = self.0;

        match v {
            0x00..=0xFF => 2,
            0x01_00..=0xFF_FF => 3,
            0x_01_00_00..=0xFF_FF_FF => 4,
            0x_01_00_00_00..=0xFF_FF_FF_FF => 5,
            0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => 6,
            0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => 7,
            0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => 8,
            0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => 9,
        }
    }

    fn max_byte_size() -> usize {
        9
    }
}
