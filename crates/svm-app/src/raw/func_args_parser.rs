use crate::{
    error::ParseError,
    types::{WasmType, WasmValue},
};

use super::{concat_nibbles, Field, Nibble, NibbleIter};

#[derive(Debug, Clone, PartialEq)]
struct WasmValueLayout {
    ty: WasmType,

    len: usize,
}

impl From<Nibble> for WasmValueLayout {
    fn from(nibble: Nibble) -> Self {
        match nibble.inner() {
            // 32-bit args layouts:
            0b_0000_0000 => Self {
                ty: WasmType::I32,
                len: 0,
            },
            0b_0000_0001 => Self {
                ty: WasmType::I32,
                len: 1,
            },
            0b_0000_0010 => Self {
                ty: WasmType::I32,
                len: 2,
            },
            0b_0000_0011 => Self {
                ty: WasmType::I32,
                len: 3,
            },
            0b_0000_0100 => Self {
                ty: WasmType::I32,
                len: 4,
            },
            //
            // 64-bit args layouts:
            0b_0000_1000 => Self {
                ty: WasmType::I64,
                len: 1,
            },
            0b_0000_1001 => Self {
                ty: WasmType::I64,
                len: 2,
            },
            0b_0000_1010 => Self {
                ty: WasmType::I64,
                len: 3,
            },
            0b_0000_1011 => Self {
                ty: WasmType::I64,
                len: 4,
            },
            0b_0000_1100 => Self {
                ty: WasmType::I64,
                len: 5,
            },
            0b_0000_1101 => Self {
                ty: WasmType::I64,
                len: 6,
            },
            0b_0000_1110 => Self {
                ty: WasmType::I64,
                len: 7,
            },
            0b_0000_1111 => Self {
                ty: WasmType::I64,
                len: 8,
            },
            //
            // special-cases
            0b_0000_0101 => Self {
                ty: WasmType::I64,
                len: 0,
            },
            _ => unreachable!(),
        }
    }
}

pub fn parse_func_args(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    let mut func_args = Vec::new();
    let layouts = parse_func_args_layout(iter)?;

    for layout in layouts.iter() {
        let arg = read_func_arg(layout, iter)?;

        func_args.push(arg);
    }

    Ok(func_args)
}

fn read_func_arg(layout: &WasmValueLayout, iter: &mut NibbleIter) -> Result<WasmValue, ParseError> {
    let n = layout.len;

    // `n` bytes <=> `2 * n` nibbles
    let nibbles = iter.take(2 * n).collect::<Vec<Nibble>>();

    let (bytes, rem) = concat_nibbles(&nibbles[..]);

    // `rem` is expected to be `None` since we've asked
    // for an even number of nibbles (= `2 * n`)
    assert!(rem.is_none());

    if bytes.len() != n {
        todo!()
    };

    let val = {
        match n {
            0..=4 => {
                let mut be_bytes: [u8; 4] = [0; 4];

                let src = bytes.as_ptr();
                let dst = unsafe { be_bytes.as_mut_ptr().offset((4 - n) as isize) };

                unsafe {
                    std::ptr::copy(src, dst, n);
                }

                let val = u32::from_be_bytes(be_bytes);

                match layout.ty {
                    WasmType::I32 => WasmValue::I32(val),
                    WasmType::I64 => WasmValue::I64(val as u64),
                }
            }
            5..=8 => {
                let mut be_bytes: [u8; 8] = [0; 8];

                let src = bytes.as_ptr();
                let dst = unsafe { be_bytes.as_mut_ptr().offset((8 - n) as isize) };

                unsafe {
                    std::ptr::copy(src, dst, n);
                }

                let val = u64::from_be_bytes(be_bytes);

                match layout.ty {
                    WasmType::I32 => unreachable!(),
                    WasmType::I64 => WasmValue::I64(val),
                }
            }
            _ => unreachable!(),
        }
    };

    Ok(val)
}

fn parse_func_args_layout(iter: &mut NibbleIter) -> Result<Vec<WasmValueLayout>, ParseError> {
    let mut args_layout = Vec::new();
    let mut has_more = true;

    while has_more {
        let nibble = iter.next();

        if let Some(nibble) = nibble {
            match nibble.inner() {
                0b_0000_0111 => {
                    // invalid input
                    return Err(ParseError::InvalidFuncArgLayout(0b_0000_0111));
                }
                0b_0000_0110 => {
                    // there are no more func args
                    has_more = false;
                }
                _ => {
                    let layout = nibble.into();
                    args_layout.push(layout);
                }
            }
        } else {
            // missing `no more func args` mark.
            return Err(ParseError::EmptyField(Field::FuncArgsNoMoreMark));
        }
    }

    Ok(args_layout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{nib, raw::concat_nibbles};

    // special-cases
    static NO_MORE: u8 = 0b_0000_0110;
    static INVALID: u8 = 0b_0000_0111;

    // i32-layout
    static I32_0B: u8 = 0b_0000_0000;
    static I32_1B: u8 = 0b_0000_0001;
    static I32_2B: u8 = 0b_0000_0010;
    static I32_3B: u8 = 0b_0000_0011;
    static I32_4B: u8 = 0b_0000_0100;

    // i64-layout
    static I64_0B: u8 = 0b_0000_0101;
    static I64_1B: u8 = 0b_0000_1000;
    static I64_2B: u8 = 0b_0000_1001;
    static I64_3B: u8 = 0b_0000_1010;
    static I64_4B: u8 = 0b_0000_1011;
    static I64_5B: u8 = 0b_0000_1100;
    static I64_6B: u8 = 0b_0000_1101;
    static I64_7B: u8 = 0b_0000_1110;
    static I64_8B: u8 = 0b_0000_1111;

    fn assert_func_args(nibbles: Vec<Nibble>, expected: Vec<WasmValue>) {
        assert!(nibbles.len() % 2 == 0);

        let (data, rem) = concat_nibbles(&nibbles[..]);
        assert!(rem.is_none());

        let mut iter = NibbleIter::new(&data[..]);

        let actual = parse_func_args(&mut iter).unwrap();

        assert_eq!(expected, actual);
    }

    fn assert_func_args_err(nibbles: Vec<Nibble>, expected: ParseError) {
        let (data, rem) = concat_nibbles(&nibbles[..]);
        assert!(rem.is_none());

        let mut iter = NibbleIter::new(&data);

        let expected = Err(expected);
        let actual = parse_func_args(&mut iter);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_func_args_zero_args() {
        let data = vec![NO_MORE << 4];
        let mut iter = NibbleIter::new(&data);

        let args = parse_func_args(&mut iter).unwrap();
        assert!(args.is_empty());
    }

    #[test]
    fn parse_func_args_zero_args_missing_no_more_mark() {
        assert_func_args_err(vec![], ParseError::EmptyField(Field::FuncArgsNoMoreMark));
    }

    #[test]
    fn parse_func_args_invalid_arg() {
        let nibbles = vec![nib!(INVALID), nib!(NO_MORE)];
        let expected = ParseError::InvalidFuncArgLayout(0b_0000_0111);

        assert_func_args_err(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_0_bytes() {
        let nibbles = vec![nib!(I32_0B), nib!(NO_MORE)];
        let expected = vec![WasmValue::I32(0)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_1_byte() {
        let nibbles = vec![nib!(I32_1B), nib!(NO_MORE), nib!(0x0A), nib!(0x0B)];
        let expected = vec![WasmValue::I32(0xAB)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_2_bytes() {
        let nibbles = vec![
            nib!(I32_2B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
        ];

        let expected = vec![WasmValue::I32(0xABCD)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_3_bytes() {
        let nibbles = vec![
            nib!(I32_3B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
        ];

        let expected = vec![WasmValue::I32(0xABCDEF)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_4_bytes() {
        let nibbles = vec![
            nib!(I32_4B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            nib!(0x01),
            nib!(0x02),
        ];

        let expected = vec![WasmValue::I32(0xABCDEF12)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_0_bytes() {
        let nibbles = vec![nib!(I64_0B), nib!(NO_MORE)];
        let expected = vec![WasmValue::I64(0)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_1_byte() {
        let nibbles = vec![nib!(I64_1B), nib!(NO_MORE), nib!(0x0A), nib!(0x0B)];

        let expected = vec![WasmValue::I64(0x0AB)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_2_bytes() {
        let nibbles = vec![
            nib!(I64_2B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
        ];

        let expected = vec![WasmValue::I64(0x0ABCD)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_3_bytes() {
        let nibbles = vec![
            nib!(I64_3B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_4_bytes() {
        let nibbles = vec![
            nib!(I64_4B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            nib!(0x01),
            nib!(0x02),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF12)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_5_bytes() {
        let nibbles = vec![
            nib!(I64_5B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            nib!(0x01),
            nib!(0x02),
            nib!(0x03),
            nib!(0x04),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF1234)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_6_bytes() {
        let nibbles = vec![
            nib!(I64_6B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            nib!(0x01),
            nib!(0x02),
            nib!(0x03),
            nib!(0x04),
            nib!(0x05),
            nib!(0x06),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF123456)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_7_bytes() {
        let nibbles = vec![
            nib!(I64_7B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            nib!(0x01),
            nib!(0x02),
            nib!(0x03),
            nib!(0x04),
            nib!(0x05),
            nib!(0x06),
            nib!(0x07),
            nib!(0x08),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF12345678)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_8_bytes() {
        let nibbles = vec![
            nib!(I64_8B),
            nib!(NO_MORE),
            nib!(0x0A),
            nib!(0x0B),
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            nib!(0x01),
            nib!(0x02),
            nib!(0x03),
            nib!(0x04),
            nib!(0x05),
            nib!(0x06),
            nib!(0x07),
            nib!(0x08),
            nib!(0x09),
            nib!(0x0A),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF123456789A)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_multiple_i32_args() {
        let nibbles = vec![
            nib!(I32_0B),  // 1st arg consumes 0 bytes
            nib!(I32_1B),  // 2st arg consumes 1 byte
            nib!(I32_2B),  // 3nd arg consumes 2 bytes
            nib!(I32_3B),  // 4th arg consumes 3 bytes
            nib!(NO_MORE), // end-of func args layouts marker
            //
            // 1st arg
            // (has no bytes)
            //
            // 2nd arg
            nib!(0x0A),
            nib!(0x0B),
            //
            // 3rd arg
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            //
            // 4th arg
            nib!(0x01),
            nib!(0x02),
            nib!(0x03),
            nib!(0x04),
            nib!(0x05),
            nib!(0x06),
            //
            // subsequent nibbles (not relevant to the func args).
            // (we use an even-length `nibbles` to simplify the test).
            nib!(0x0F),
            nib!(0x0F),
            nib!(0x0F),
        ];

        let expected = vec![
            WasmValue::I32(0),
            WasmValue::I32(0xAB),
            WasmValue::I32(0xCDEF),
            WasmValue::I32(0x123456),
        ];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_multiple_i64_args() {
        let nibbles = vec![
            nib!(I64_0B),  // 1st arg consumes 0 bytes
            nib!(I64_1B),  // 2st arg consumes 1 byte
            nib!(I64_2B),  // 3nd arg consumes 2 bytes
            nib!(I64_3B),  // 4th arg consumes 3 bytes
            nib!(NO_MORE), // end-of func args layouts marker
            //
            // 1st arg
            // (has no bytes)
            //
            // 2nd arg
            nib!(0x0A),
            nib!(0x0B),
            //
            // 3rd arg
            nib!(0x0C),
            nib!(0x0D),
            nib!(0x0E),
            nib!(0x0F),
            //
            // 4th arg
            nib!(0x01),
            nib!(0x02),
            nib!(0x03),
            nib!(0x04),
            nib!(0x05),
            nib!(0x06),
            //
            // subsequent nibbles (not relevant to the func args).
            // (we use an even-length `nibbles` to simplify the test).
            nib!(0x0F),
            nib!(0x0F),
            nib!(0x0F),
        ];

        let expected = vec![
            WasmValue::I64(0),
            WasmValue::I64(0xAB),
            WasmValue::I64(0xCDEF),
            WasmValue::I64(0x123456),
        ];

        assert_func_args(nibbles, expected);
    }
}
