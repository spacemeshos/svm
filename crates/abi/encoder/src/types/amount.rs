use crate::{ByteSize, Encoder};

use svm_sdk_types::Amount;

macro_rules! encode {
    ( $W:ty) => {
        impl Encoder<$W> for Amount {
            fn encode(&self, w: &mut $W) {
                let v = self.0;
                let size = self.byte_size();

                use svm_abi_layout::layout;

                match size {
                    2 => {
                        w.push(layout::AMOUNT_1B);
                        w.push(v as u8);
                    }
                    3 => {
                        w.push(layout::AMOUNT_2B);

                        let bytes: [u8; 2] = (v as u16).to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                    }
                    4 => {
                        w.push(layout::AMOUNT_3B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                    }
                    5 => {
                        w.push(layout::AMOUNT_4B);

                        let bytes: [u8; 4] = (v as u32).to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                    }
                    6 => {
                        w.push(layout::AMOUNT_5B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);
                        debug_assert_eq!(bytes[2], 0);

                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    7 => {
                        w.push(layout::AMOUNT_6B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);
                        debug_assert_eq!(bytes[1], 0);

                        w.push(bytes[2]);
                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    8 => {
                        w.push(layout::AMOUNT_7B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        debug_assert_eq!(bytes[0], 0);

                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    9 => {
                        w.push(layout::AMOUNT_8B);

                        let bytes: [u8; 8] = v.to_be_bytes();

                        w.push(bytes[0]);
                        w.push(bytes[1]);
                        w.push(bytes[2]);
                        w.push(bytes[3]);
                        w.push(bytes[4]);
                        w.push(bytes[5]);
                        w.push(bytes[6]);
                        w.push(bytes[7]);
                    }
                    _ => svm_sdk_std::panic(),
                }
            }
        }
    };
}

encode!(svm_sdk_std::Vec<u8>);

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
