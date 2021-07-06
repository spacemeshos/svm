use svm_abi_layout::layout;

use crate::{traits::Push, ByteSize, Encoder};

impl<T, W> Encoder<W> for T
where
    T: Integer16 + ByteSize,
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        let v = u16::from_be_bytes((*self).to_be_bytes());

        let size = self.byte_size();

        match size {
            2 => {
                w.push(Self::LAYOUT_1B);
                w.push(v as u8);
            }
            _ => {
                w.push(Self::LAYOUT_2B);

                let bytes: [u8; 2] = v.to_be_bytes();

                w.push(bytes[0]);
                w.push(bytes[1]);
            }
        }
    }
}

pub trait Integer16: Copy {
    const LAYOUT_1B: u8;
    const LAYOUT_2B: u8;

    fn to_be_bytes(self) -> [u8; 2];
}

impl Integer16 for i16 {
    const LAYOUT_1B: u8 = layout::I16_1B;
    const LAYOUT_2B: u8 = layout::I16_2B;

    fn to_be_bytes(self) -> [u8; 2] {
        i16::to_be_bytes(self)
    }
}

impl Integer16 for u16 {
    const LAYOUT_1B: u8 = layout::U16_1B;
    const LAYOUT_2B: u8 = layout::U16_2B;

    fn to_be_bytes(self) -> [u8; 2] {
        u16::to_be_bytes(self)
    }
}

macro_rules! encode_byte_size {
    ($ty:ty) => {
        impl ByteSize for $ty {
            #[inline]
            fn byte_size(&self) -> usize {
                let v = *self as u16;

                match v {
                    0..=0xFF => 2,
                    _ => 3,
                }
            }

            fn max_byte_size() -> usize {
                3
            }
        }
    };
}

encode_byte_size!(i16);
encode_byte_size!(u16);
