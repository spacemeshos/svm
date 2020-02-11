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
    let mut last_layout = false;
    let mut no_more_layouts = false;

    while no_more_layouts {
        let nibble = iter.next();

        if last_layout {
            no_more_layouts = true;
        }

        if let Some(nibble) = nibble {
            match nibble.0 {
                0b_0000_0111 => {
                    // nibble `01111` denotes: `func has no args`

                    if args_layout.len() > 0 {
                        panic!()
                    }

                    no_more_layouts = true;
                }
                0b_0000_0110 => {
                    // nibble `0110` denotes: `next func arg is the last one`

                    if last_layout {
                        panic!()
                    }

                    // next func arg will be the last one
                    last_layout = true;
                }
                _ => {
                    let layout = nibble.into();
                    args_layout.push(layout);
                }
            }
        } else {
            panic!()
        }
    }

    Ok(args_layout)
}

#[cfg(test)]
mod tests {
    use super::*;

    static NO_MORE: u8 = 0b_0000_0110;
    static INVALID: u8 = 0b_0000_0111;
    static I32_0B: u8 = 0b_0000_0000;
    static I32_1B: u8 = 0b_0000_0001;
    static I32_2B: u8 = 0b_0000_0010;
    static I32_3B: u8 = 0b_0000_0011;
    static I32_4B: u8 = 0b_0000_0100;

    #[test]
    fn parse_func_args_i32_no_args() {
        todo!()
        // let data = vec![NO_ARGS];
        // let mut iter = NibbleIter::new(&data);

        // let actual = parse_func_args(&mut iter).unwrap();
        // let expected = vec![WasmValue::I32(0)];

        // assert_eq!(expected, actual);
    }

    #[test]
    fn parse_func_args_i32_arg_0_bytes() {
        todo!()
        // let data = vec![arg!(I32_0B];
        // let mut iter = NibbleIter::new(&data);

        // let actual = parse_func_args(&mut iter).unwrap();
        // let expected = vec![WasmValue::I32(0)];

        // assert_eq!(expected, actual);
    }
}
