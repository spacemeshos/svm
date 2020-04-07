use crate::{error::ParseError, types::WasmValue};

use super::super::{
    wasm::{decode_wasm_value, WasmValueLayout, DO_SKIP, NO_MORE},
    Field, NibbleIter,
};

/// Decodes raw func args field.
pub fn decode_func_args(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    decode_func_values(iter)
}

/// Decodes raw func returns.
pub fn decode_func_rets(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    decode_func_values(iter)
}

/// Decodes raw func values (args or returns)
fn decode_func_values(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    let mut func_values = Vec::new();
    let layouts = decode_values_layout(iter)?;

    for layout in layouts.iter() {
        let val = decode_wasm_value(layout, iter)?;

        func_values.push(val);
    }

    Ok(func_values)
}

fn decode_values_layout(iter: &mut NibbleIter) -> Result<Vec<WasmValueLayout>, ParseError> {
    let mut args_layout = Vec::new();
    let mut has_more = true;

    while has_more {
        let nibble = iter.next();

        if let Some(nibble) = nibble {
            match nibble.inner() {
                NO_MORE => {
                    // marker denoting: "there are no more func args"
                    has_more = false;
                }
                DO_SKIP => {
                    // marker denoting: "ignore, skip to next nibble".
                    //
                    // should be used to align the func args layouts offset,
                    // so that each arg layout will start at an even position.
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
    use crate::{
        nib,
        raw::{concat_nibbles, wasm::*, Nibble},
    };

    fn assert_func_args(nibbles: Vec<Nibble>, expected: Vec<WasmValue>) {
        assert!(nibbles.len() % 2 == 0);

        let (data, rem) = concat_nibbles(&nibbles[..]);
        assert!(rem.is_none());

        let mut iter = NibbleIter::new(&data[..]);

        let actual = decode_func_args(&mut iter).unwrap();

        assert_eq!(expected, actual);
    }

    fn assert_func_args_err(nibbles: Vec<Nibble>, expected: ParseError) {
        let (data, rem) = concat_nibbles(&nibbles[..]);
        assert!(rem.is_none());

        let mut iter = NibbleIter::new(&data);

        let expected = Err(expected);
        let actual = decode_func_args(&mut iter);

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_func_args_zero_args_missing_no_more_marker() {
        assert_func_args_err(vec![], ParseError::EmptyField(Field::FuncArgsNoMoreMark));
    }

    #[test]
    fn decode_func_args_i32_arg_0_bytes() {
        let nibbles = vec![nib!(I32_0B), nib!(NO_MORE)];
        let expected = vec![WasmValue::I32(0)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn decode_func_args_i32_arg_1_byte() {
        let nibbles = vec![nib!(I32_1B), nib!(NO_MORE), nib!(0x0A), nib!(0x0B)];
        let expected = vec![WasmValue::I32(0xAB)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn decode_func_args_i32_arg_2_bytes() {
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
    fn decode_func_args_i32_arg_3_bytes() {
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
    fn decode_func_args_i32_arg_4_bytes() {
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
    fn decode_func_args_i64_arg_0_bytes() {
        let nibbles = vec![nib!(I64_0B), nib!(NO_MORE)];
        let expected = vec![WasmValue::I64(0)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn decode_func_args_i64_arg_1_byte() {
        let nibbles = vec![nib!(I64_1B), nib!(NO_MORE), nib!(0x0A), nib!(0x0B)];

        let expected = vec![WasmValue::I64(0x0AB)];

        assert_func_args(nibbles, expected);
    }

    #[test]
    fn decode_func_args_i64_arg_2_bytes() {
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
    fn decode_func_args_i64_arg_3_bytes() {
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
    fn decode_func_args_i64_arg_4_bytes() {
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
    fn decode_func_args_i64_arg_5_bytes() {
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
    fn decode_func_args_i64_arg_6_bytes() {
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
    fn decode_func_args_i64_arg_7_bytes() {
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
    fn decode_func_args_i64_arg_8_bytes() {
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
    fn decode_func_args_multiple_i32_args() {
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
    fn decode_func_args_multiple_i64_args() {
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

    #[test]
    fn decode_func_args_missing_some_arg_values_bytes() {
        let nibbles = vec![
            nib!(I32_2B),
            nib!(NO_MORE),
            //
            // arg contains only 2 nibbles
            // instead of 4 (since layout states `2 bytes`)
            nib!(0x0A),
            nib!(0x0B),
        ];

        let expected = ParseError::IncompleteWasmValue {
            expected_nibbles: 4,
            actual_read: 2,
        };

        assert_func_args_err(nibbles, expected);
    }

    #[test]
    fn decode_func_args_skip_marker() {
        let nibbles = vec![
            nib!(I32_1B),
            nib!(DO_SKIP),
            nib!(NO_MORE),
            // arg func (one byte)
            nib!(0x0A),
            nib!(0x0B),
            // after arg data
            nib!(0x0F),
            nib!(0x0F),
            nib!(0x0F),
        ];

        let expected = vec![WasmValue::I32(0xAB)];

        assert_func_args(nibbles, expected);
    }
}
