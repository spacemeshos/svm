use num_traits::{AsPrimitive, Bounded};

use crate::traits::{Numeric, Push};
use crate::{ABIEncoder, ByteSize};

impl<T> ABIEncoder for T
where
    T: Numeric + ByteSize + num_traits::PrimInt,
{
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        // We need three pieces of information to generate the layout marker
        // bytes on the fly:
        //
        // 1. Sign. Is the number type signed or unsigned?
        // 2. Max. width. How many bytes is it? (e.g. u32 is 4 bytes).
        // 3. Actual width. How many bytes do we need to store this specific
        //    number? (e.g. 0-127 only takes one byte, regardless of the number
        //    type).
        let type_is_signed = T::min_value() < T::zero();

        // We get the payload size by subtracing 1, which is the space required
        // by the layout marker byte. Same for the max. width.
        let payload_size = self.byte_size() - 1;
        let layout_marker = layout_integer(
            T::max_byte_size() as u32 - 1,
            payload_size as u32,
            type_is_signed,
        );
        w.push(layout_marker);

        // Encoding is tricky and requires first to cast to the relevant
        // unsigned type, then extending to 64 bits. Note: you can't cast
        // directly as some numbers will result in different representations.
        let self_unsigned: T::Unsigned = self.as_();
        let self_u64: u64 = self_unsigned.as_();
        let bytes: [u8; 8] = self_u64.to_be_bytes();

        // Finally, push all relevant bytes.
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
    fn max_byte_size() -> usize {
        Self::max_value().byte_size()
    }

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

/// Integer layout marker bytes generator. It looks scary, but all it does is it
/// dynamically encodes the layout formula specificied by the ABI.
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
        assert_eq!(layout_integer(2, 2, true), layout::I16_2B);
        assert_eq!(layout_integer(2, 1, false), layout::U16_1B);
        assert_eq!(layout_integer(2, 2, false), layout::U16_2B);
        assert_eq!(layout_integer(4, 1, true), layout::I32_1B);
        assert_eq!(layout_integer(4, 2, true), layout::I32_2B);
        assert_eq!(layout_integer(4, 3, true), layout::I32_3B);
        assert_eq!(layout_integer(4, 4, true), layout::I32_4B);
        assert_eq!(layout_integer(4, 1, false), layout::U32_1B);
        assert_eq!(layout_integer(4, 2, false), layout::U32_2B);
        assert_eq!(layout_integer(4, 3, false), layout::U32_3B);
        assert_eq!(layout_integer(4, 4, false), layout::U32_4B);
        assert_eq!(layout_integer(8, 1, true), layout::I64_1B);
        assert_eq!(layout_integer(8, 2, true), layout::I64_2B);
        assert_eq!(layout_integer(8, 3, true), layout::I64_3B);
        assert_eq!(layout_integer(8, 4, true), layout::I64_4B);
        assert_eq!(layout_integer(8, 5, true), layout::I64_5B);
        assert_eq!(layout_integer(8, 6, true), layout::I64_6B);
        assert_eq!(layout_integer(8, 7, true), layout::I64_7B);
        assert_eq!(layout_integer(8, 8, true), layout::I64_8B);
        assert_eq!(layout_integer(8, 1, false), layout::U64_1B);
        assert_eq!(layout_integer(8, 2, false), layout::U64_2B);
        assert_eq!(layout_integer(8, 3, false), layout::U64_3B);
        assert_eq!(layout_integer(8, 4, false), layout::U64_4B);
        assert_eq!(layout_integer(8, 5, false), layout::U64_5B);
        assert_eq!(layout_integer(8, 6, false), layout::U64_6B);
        assert_eq!(layout_integer(8, 7, false), layout::U64_7B);
        assert_eq!(layout_integer(8, 8, false), layout::U64_8B);
    }
}
