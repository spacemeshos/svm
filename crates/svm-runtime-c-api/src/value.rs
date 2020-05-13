use std::convert::TryFrom;
use std::io::{self, Cursor, ErrorKind};

use svm_app::types::{WasmType, WasmValue};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::svm_byte_array;

///
/// This file contains the implementation of encoding & decoding of a `Vec<WasmValue>` into `svm_byte_array`.
/// (and vice-versa).
///
/// This encoding (and decoding) functionality should be also implemented by any SVM clients (e.g: C, Golang).
/// The design motivation is sticking with `svm_byte_array` as the mechanism for passing data between SVM client
/// to SVM (via the `SVM C-API``)
///
////
/// ### Encoding format:
///
/// * First byte is for the number of WASM values.
///
/// * Then each WASM value is encoded as:
///   - A WASM value type. It can be `I32` or `I64` (SVM doesn't support Floats).
///     The encoding is implemented in `WasmType` under the `svm-app` crate.
///   - A Big-Endian encoding of the WASM value. `I32` consumes 4 bytes and `I64` 8 bytes.
///
/// +----------------------------------------------------------------------+
/// | #values  | value #1  |  value #1 |        |  value #N   |  value #N  |
/// | (1 byte) |  type     |  (4 or 8  | . . .  |    type     |   (4 or 8  |
/// |          | (1 byte)  |   bytes)  |        |   (1 byte)  |   bytes)   |
/// +----------+--------------------------------+-------------+------------+
///

/// Converts `svm_byte_array` into `Vec<WasmerValue>`
///
/// ```
/// use std::io;
/// use std::convert::TryFrom;
///
/// use svm_app::types::WasmValue;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let values = vec![WasmValue::I32(5), WasmValue::I64(10)];
///
/// let bytes: svm_byte_array = (&values).into();
/// let vec: Result<_, io::Error> = Vec::<WasmValue>::try_from(bytes);
///
/// assert_eq!(vec.unwrap(), values);
/// ```
impl From<&[WasmValue]> for svm_byte_array {
    fn from(values: &[WasmValue]) -> svm_byte_array {
        let nvalues = values.len();
        assert!(nvalues <= std::u8::MAX as usize);

        let capacity = 1 + nvalues * 9;

        let mut bytes = Vec::with_capacity(capacity);

        bytes.write_u8(nvalues as u8);

        for value in values.iter() {
            match value {
                WasmValue::I32(v) => {
                    bytes.write_u8(WasmType::I32.into());
                    bytes.write_u32::<BigEndian>(*v);
                }
                WasmValue::I64(v) => {
                    bytes.write_u8(WasmType::I64.into());
                    bytes.write_u64::<BigEndian>(*v);
                }
            }
        }

        bytes.into()
    }
}

impl From<Vec<WasmValue>> for svm_byte_array {
    #[inline]
    fn from(values: Vec<WasmValue>) -> svm_byte_array {
        (&values[..]).into()
    }
}

impl From<&Vec<WasmValue>> for svm_byte_array {
    #[inline]
    fn from(values: &Vec<WasmValue>) -> svm_byte_array {
        (&values[..]).into()
    }
}

impl TryFrom<svm_byte_array> for Vec<WasmValue> {
    type Error = io::Error;

    fn try_from(bytes: svm_byte_array) -> Result<Self, Self::Error> {
        let slice: &[u8] =
            unsafe { std::slice::from_raw_parts(bytes.bytes, bytes.length as usize) };

        let length = slice.len();
        if length == 0 {
            return Err(ErrorKind::InvalidInput.into());
        }

        let nvalues = slice[0];

        let mut values = Vec::with_capacity(nvalues as usize);
        let mut cursor = Cursor::new(&slice[1..]);

        for _ in 0..nvalues {
            let ty = cursor.read_u8()?;
            let ty = WasmType::try_from(ty);

            let value = match ty {
                Ok(WasmType::I32) => {
                    let value = cursor.read_u32::<BigEndian>()?;
                    WasmValue::I32(value)
                }
                Ok(WasmType::I64) => {
                    let value = cursor.read_u64::<BigEndian>()?;
                    WasmValue::I64(value)
                }
                _ => return Err(ErrorKind::InvalidInput.into()),
            };

            values.push(value);
        }

        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec_values_to_svm_byte_array() {
        let vec = Vec::<WasmValue>::new();

        let bytes: svm_byte_array = vec.into();
        let slice: &[u8] = bytes.into();

        let nvalues = slice[0];
        assert_eq!(nvalues, 0);
    }

    #[test]
    fn empty_svm_byte_array_to_vec_values_errors() {
        let bytes = svm_byte_array {
            bytes: std::ptr::null(),
            length: 0,
        };

        let res: Result<Vec<WasmValue>, io::Error> = Vec::try_from(bytes);

        assert!(res.is_err());
    }

    #[test]
    fn svm_byte_array_to_vec_values_with_zero_items() {
        let raw = vec![0];

        let bytes = svm_byte_array {
            bytes: raw.as_ptr(),
            length: raw.len() as u32,
        };

        let res: Result<Vec<WasmValue>, io::Error> = Vec::try_from(bytes);
        assert_eq!(res.unwrap(), vec![]);
    }

    #[test]
    fn svm_byte_array_to_vec_values_with_missing_type_byte_error() {
        let raw = vec![1];

        let bytes = svm_byte_array {
            bytes: raw.as_ptr(),
            length: raw.len() as u32,
        };

        let res: Result<Vec<WasmValue>, io::Error> = Vec::try_from(bytes);
        assert!(res.is_err());
    }

    #[test]
    fn svm_byte_array_to_vec_values_with_missing_value_bytes_error() {
        let raw = vec![1, WasmType::I32.into(), 0x10, 0x20];

        let bytes = svm_byte_array {
            bytes: raw.as_ptr(),
            length: raw.len() as u32,
        };

        let res: Result<Vec<WasmValue>, io::Error> = Vec::try_from(bytes);
        assert!(res.is_err());
    }
}
