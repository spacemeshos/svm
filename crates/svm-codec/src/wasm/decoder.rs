use svm_nibble::{concat_nibbles, Nibble, NibbleIter};
use svm_types::{WasmType, WasmValue};

use crate::{error::ParseError, wasm::WasmValueLayout};

/// Decodes a wasm value given its expected `layout`.
pub fn decode_wasm_value(
    layout: &WasmValueLayout,
    iter: &mut NibbleIter,
) -> Result<WasmValue, ParseError> {
    let n = layout.len;

    // `n` bytes <=> `2 * n` nibbles
    let nibbles = iter.take(2 * n).collect::<Vec<Nibble>>();

    let (bytes, rem) = concat_nibbles(&nibbles[..]);

    if bytes.len() != n {
        let mut actual_read = bytes.len() * 2;

        if rem.is_some() {
            actual_read += 1;
        }

        return Err(ParseError::IncompleteWasmValue {
            actual_read,
            expected_nibbles: 2 * n,
        });
    };

    // `rem` is expected to be `None` since we've asked
    // for an even number of nibbles (= `2 * n`)
    assert!(rem.is_none());

    let val = {
        match n {
            0..=4 => {
                let mut be_bytes: [u8; 4] = [0; 4];

                let src = bytes.as_ptr();
                let dst = unsafe { be_bytes.as_mut_ptr().add(4 - n) };

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
                let dst = unsafe { be_bytes.as_mut_ptr().add(8 - n) };

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
