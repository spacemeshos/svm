use crate::{
    error::ParseError,
    types::{WasmType, WasmValue},
};

use super::{Field, Nibble, NibbleIter};

#[derive(Debug, Clone, PartialEq)]
struct WasmValueLayout {
    ty: WasmType,

    len: usize,
}

impl From<Nibble> for WasmValueLayout {
    fn from(nibble: Nibble) -> Self {
        match nibble.0 {
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

    let bytes = iter.read_bytes(n);

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
            match nibble.0 {
                0b_0000_0111 => {
                    // invalid input
                    return Err(ParseError::InvalidFuncArgLayout(0b_0000_0111));
                }
                0b_0000_0110 => {
                    // next func arg will be the last one
                    has_more = false;
                }
                _ => {
                    let layout = nibble.into();
                    args_layout.push(layout);
                }
            }
        } else {
            return Err(ParseError::EmptyField(Field::FuncArgsNoMoreMark));
        }
    }

    Ok(args_layout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::concat_nibbles;

    // special cases
    static NO_MORE: Nibble = Nibble(0b_0000_0110);
    static INVALID: Nibble = Nibble(0b_0000_0111);

    // i32-layout
    static I32_0B: Nibble = Nibble(0b_0000_0000);
    static I32_1B: Nibble = Nibble(0b_0000_0001);
    static I32_2B: Nibble = Nibble(0b_0000_0010);
    static I32_3B: Nibble = Nibble(0b_0000_0011);
    static I32_4B: Nibble = Nibble(0b_0000_0100);

    // i64-layout
    static I64_0B: Nibble = Nibble(0b_0000_0101);
    static I64_1B: Nibble = Nibble(0b_0000_1000);
    static I64_2B: Nibble = Nibble(0b_0000_1001);
    static I64_3B: Nibble = Nibble(0b_0000_1010);
    static I64_4B: Nibble = Nibble(0b_0000_1011);
    static I64_5B: Nibble = Nibble(0b_0000_1100);
    static I64_6B: Nibble = Nibble(0b_0000_1101);
    static I64_7B: Nibble = Nibble(0b_0000_1110);
    static I64_8B: Nibble = Nibble(0b_0000_1111);

    fn assert_func_args(nibbles: Vec<Nibble>, expected: Vec<WasmValue>) {
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
        let data = vec![(NO_MORE.0 << 4)];
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
        let nibbles = vec![INVALID, NO_MORE];
        let expected = ParseError::InvalidFuncArgLayout(0b_0000_0111);

        assert_func_args_err(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_0_bytes() {
        let nibbles = vec![I32_0B, NO_MORE];
        let expected = vec![WasmValue::I32(0)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_1_byte() {
        let nibbles = vec![I32_1B, NO_MORE, Nibble(0x0A), Nibble(0x0B)];
        let expected = vec![WasmValue::I32(0xAB)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_2_bytes() {
        let nibbles = vec![
            I32_2B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
        ];

        let expected = vec![WasmValue::I32(0xABCD)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_3_bytes() {
        let nibbles = vec![
            I32_3B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
        ];

        let expected = vec![WasmValue::I32(0xABCDEF)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i32_arg_4_bytes() {
        let nibbles = vec![
            I32_4B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
            Nibble(0x01),
            Nibble(0x02),
        ];

        let expected = vec![WasmValue::I32(0xABCDEF12)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_0_bytes() {
        let nibbles = vec![I64_0B, NO_MORE];
        let expected = vec![WasmValue::I64(0)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_1_byte() {
        let nibbles = vec![I64_1B, NO_MORE, Nibble(0x0A), Nibble(0x0B)];

        let expected = vec![WasmValue::I64(0x0AB)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_2_bytes() {
        let nibbles = vec![
            I64_2B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
        ];

        let expected = vec![WasmValue::I64(0x0ABCD)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_3_bytes() {
        let nibbles = vec![
            I64_3B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_4_bytes() {
        let nibbles = vec![
            I64_4B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
            Nibble(0x01),
            Nibble(0x02),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF12)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_5_bytes() {
        let nibbles = vec![
            I64_5B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
            Nibble(0x01),
            Nibble(0x02),
            Nibble(0x03),
            Nibble(0x04),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF1234)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_6_bytes() {
        let nibbles = vec![
            I64_6B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
            Nibble(0x01),
            Nibble(0x02),
            Nibble(0x03),
            Nibble(0x04),
            Nibble(0x05),
            Nibble(0x06),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF123456)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_7_bytes() {
        let nibbles = vec![
            I64_7B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
            Nibble(0x01),
            Nibble(0x02),
            Nibble(0x03),
            Nibble(0x04),
            Nibble(0x05),
            Nibble(0x06),
            Nibble(0x07),
            Nibble(0x08),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF12345678)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn parse_func_args_i64_arg_8_bytes() {
        let nibbles = vec![
            I64_8B,
            NO_MORE,
            Nibble(0x0A),
            Nibble(0x0B),
            Nibble(0x0C),
            Nibble(0x0D),
            Nibble(0x0E),
            Nibble(0x0F),
            Nibble(0x01),
            Nibble(0x02),
            Nibble(0x03),
            Nibble(0x04),
            Nibble(0x05),
            Nibble(0x06),
            Nibble(0x07),
            Nibble(0x08),
            Nibble(0x09),
            Nibble(0x0A),
        ];

        let expected = vec![WasmValue::I64(0x0ABCDEF123456789A)];

        assert_func_args(nibbles, expected);
    }
}
