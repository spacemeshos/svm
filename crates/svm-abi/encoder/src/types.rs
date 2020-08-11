use svm_nibble::{Nibble, NibbleWriter};

use crate::{layout, Encoder};

impl Encoder for u8 {
    fn encode(&self, w: &mut NibbleWriter) {
        let ty = match self {
            0 => layout::I32_0B,
            _ => layout::I32_1B,
        };
        w.write_byte(ty);

        if *self != 0 {
            w.write_byte(*self);
        }
    }
}

impl Encoder for i8 {
    #[inline]
    fn encode(&self, w: &mut NibbleWriter) {
        let n = *self as u8;
        <u8 as Encoder>::encode(&n, w);
    }
}

impl Encoder for u16 {
    fn encode(&self, w: &mut NibbleWriter) {
        match self {
            0..=0xFF => {
                let n = *self as u8;
                <u8 as Encoder>::encode(&n, w);
            }
            _ => {
                w.write_byte(layout::I32_2B);

                let bytes: [u8; 2] = self.to_be_bytes();
                w.write_bytes(&bytes)
            }
        };
    }
}

impl Encoder for i16 {
    #[inline]
    fn encode(&self, w: &mut NibbleWriter) {
        let n = *self as u16;
        <u16 as Encoder>::encode(&n, w);
    }
}

impl Encoder for u32 {
    fn encode(&self, w: &mut NibbleWriter) {
        match self {
            0x000..=0xFF_FF => {
                let n = *self as u16;
                <u16 as Encoder>::encode(&n, w);
            }
            0x01_00_00..=0xFF_FF_FF => {
                w.write_byte(layout::I32_3B);

                let bytes: [u8; 4] = self.to_be_bytes();

                debug_assert_eq!(bytes[0], 0);

                w.write_bytes(&bytes[1..]);
            }
            0x01_00_00_00..=0xFF_FF_FF_FF => {
                w.write_byte(layout::I32_4B);

                let bytes: [u8; 4] = self.to_be_bytes();
                w.write_bytes(&bytes);
            }
        }
    }
}

impl Encoder for i32 {
    #[inline]
    fn encode(&self, w: &mut NibbleWriter) {
        let n = *self as u32;
        <u32 as Encoder>::encode(&n, w);
    }
}

impl Encoder for u64 {
    fn encode(&self, w: &mut NibbleWriter) {
        match self {
            0 => {
                w.write_byte(layout::I64_0B);
            }
            0x01..=0xFF => {
                w.write_byte(layout::I64_1B);
                w.write_byte(*self as u8);
            }
            0x01_00..=0xFF_FF => {
                w.write_byte(layout::I64_2B);

                let bytes: [u8; 2] = (*self as u16).to_be_bytes();
                w.write_bytes(&bytes);
            }
            0x_01_00_00..=0xFF_FF_FF => {
                w.write_byte(layout::I64_3B);

                let bytes: [u8; 4] = (*self as u32).to_be_bytes();

                debug_assert_eq!(bytes[0], 0);

                w.write_bytes(&bytes[1..]);
            }
            0x_01_00_00_00..=0xFF_FF_FF_FF => {
                w.write_byte(layout::I64_4B);

                let bytes: [u8; 4] = (*self as u32).to_be_bytes();
                w.write_bytes(&bytes);
            }
            0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => {
                w.write_byte(layout::I64_5B);

                let bytes: [u8; 8] = self.to_be_bytes();

                debug_assert_eq!(bytes[0], 0);
                debug_assert_eq!(bytes[1], 0);
                debug_assert_eq!(bytes[2], 0);

                w.write_bytes(&bytes[3..]);
            }
            0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => {
                w.write_byte(layout::I64_6B);

                let bytes: [u8; 8] = self.to_be_bytes();

                debug_assert_eq!(bytes[0], 0);
                debug_assert_eq!(bytes[1], 0);

                w.write_bytes(&bytes[2..]);
            }
            0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => {
                w.write_byte(layout::I64_7B);

                let bytes: [u8; 8] = self.to_be_bytes();

                debug_assert_eq!(bytes[0], 0);

                w.write_bytes(&bytes[1..]);
            }
            0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => {
                w.write_byte(layout::I64_8B);

                let bytes: [u8; 8] = self.to_be_bytes();
                w.write_bytes(&bytes);
            }
        }
    }
}

impl Encoder for i64 {
    #[inline]
    fn encode(&self, w: &mut NibbleWriter) {
        let n = *self as u64;
        <u64 as Encoder>::encode(&n, w);
    }
}
