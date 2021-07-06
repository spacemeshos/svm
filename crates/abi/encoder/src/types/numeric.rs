use num_traits::{AsPrimitive, Bounded};
use svm_abi_layout::layout;

use crate::traits::{Numeric, Push};
use crate::{ByteSize, Encoder};

impl<T, W> Encoder<W> for T
where
    T: Numeric + ByteSize + num_traits::PrimInt,
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        let type_is_signed = T::min_value() < T::zero();

        let payload_size = self.byte_size() - 1;
        let layout_marker = layout::integer(
            T::max_byte_size() as u32 - 1,
            payload_size as u32,
            type_is_signed,
        );
        w.push(layout_marker);

        let self_unsigned: T::Unsigned = self.as_();
        let self_u64: u64 = self_unsigned.as_();
        let bytes: [u8; 8] = self_u64.to_be_bytes();

        seq_macro::seq!(i in 0..8 {
            if payload_size >= 8 - i {
                w.push(bytes[i]);
            }
        });
    }
}

impl<T> ByteSize for T
where
    T: Numeric + Bounded,
{
    fn byte_size(&self) -> usize {
        let self_unsigned: T::Unsigned = self.as_();
        let self_u64: u64 = self_unsigned.as_();

        match self_u64 {
            0..=0xFF => 2,
            0..=0xFF_FF => 3,
            0..=0xFF_FF_FF => 4,
            0..=0xFF_FF_FF_FF => 5,
            0..=0xFF_FF_FF_FF_FF => 6,
            0..=0xFF_FF_FF_FF_FF_FF => 7,
            0..=0xFF_FF_FF_FF_FF_FF_FF => 8,
            0..=0xFF_FF_FF_FF_FF_FF_FF_FF => 9,
        }
    }

    fn max_byte_size() -> usize {
        Self::max_value().byte_size()
    }
}

impl Numeric for i8 {
    type Unsigned = u8;
}

impl Numeric for u8 {
    type Unsigned = u8;
}

impl Numeric for i16 {
    type Unsigned = u16;
}

impl Numeric for u16 {
    type Unsigned = u16;
}

impl Numeric for i32 {
    type Unsigned = u32;
}

impl Numeric for u32 {
    type Unsigned = u32;
}

impl Numeric for i64 {
    type Unsigned = u64;
}

impl Numeric for u64 {
    type Unsigned = u64;
}
