use std::convert::TryFrom;
use std::io::{self, Cursor, ErrorKind};

use svm_app::types::{WasmType, WasmValue};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::svm_byte_array;

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
