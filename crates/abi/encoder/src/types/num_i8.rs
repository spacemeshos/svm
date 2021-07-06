use svm_abi_layout::layout;

use crate::{traits::Push, ByteSize, Encoder};

impl<W> Encoder<W> for u8
where
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        w.push(layout::U8);
        w.push(*self);
    }
}

impl<W> Encoder<W> for i8
where
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        w.push(layout::I8);
        w.push(*self as u8);
    }
}

macro_rules! impl_byte_size {
    ($ty:ty) => {
        impl ByteSize for $ty {
            fn byte_size(&self) -> usize {
                2
            }

            fn max_byte_size() -> usize {
                2
            }
        }
    };
}

impl_byte_size!(i8);
impl_byte_size!(u8);
