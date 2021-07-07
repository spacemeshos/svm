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
        let layout_marker = layout_integer(
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

pub fn layout_integer(max_width_in_bytes: u32, width_in_bytes: u32, signed: bool) -> u8 {
    use svm_abi_layout::layout::*;
    debug_assert!(width_in_bytes <= max_width_in_bytes);
    debug_assert!(max_width_in_bytes <= 8);
    match (max_width_in_bytes, (width_in_bytes - 1) as u8, signed) {
        (1, 0, true) => I8,
        (1, 0, false) => U8,
        (2, n, true) => I16_1B | ((n & 0b1) << 4),
        (2, n, false) => U16_1B | ((n & 0b1) << 4),
        (4, n, true) => I32_1B | ((n & 0b11) << 4),
        (4, n, false) => U32_1B | ((n & 0b11) << 4),
        (8, n, true) => I64_1B | ((n & 0b111) << 4),
        (8, n, false) => U64_1B | ((n & 0b111) << 4),
        _ => panic!("Invalid argument for layout information."),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use svm_abi_layout::layout;

    #[test]
    fn integer_layouts() {
        assert_eq!(layout_integer(1, 1, true), layout::I8);
        assert_eq!(layout_integer(1, 1, false), layout::U8);
        assert_eq!(layout_integer(2, 1, true), layout::I16_1B);
        assert_eq!(layout_integer(2, 2, false), layout::U16_2B);
        assert_eq!(layout_integer(8, 8, false), layout::U64_8B);
    }
}
