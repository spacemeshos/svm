use std::convert::TryFrom;
use std::io::{self, Cursor, ErrorKind};

use svm_app::types::WasmValue;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::svm_byte_array;

impl TryFrom<svm_byte_array> for Vec<WasmValue> {
    type Error = io::Error;

    fn try_from(bytes: svm_byte_array) -> Result<Self, Self::Error> {
        let slice: &[u8] =
            unsafe { std::slice::from_raw_parts(bytes.bytes, bytes.length as usize) };

        let length = slice.len();
        if length == 0 {
            // ErrorKind::InvalidInput.into()
            panic!("invalid input...")
        }

        let nvalues = slice[0];
        let mut values = Vec::with_capacity(nvalues as usize);

        let mut cursor = Cursor::new(&slice[1..]);

        for _ in 0..nvalues {
            let ty = cursor.read_u8().expect("invalid input...");

            let value = match ty {
                0 => {
                    let value = cursor.read_u32::<BigEndian>().unwrap();
                    WasmValue::I32(value)
                }
                1 => {
                    let value = cursor.read_u64::<BigEndian>().unwrap();
                    WasmValue::I64(value)
                }
                _ => panic!("invalid input.."),
            };

            values.push(value);
        }

        Ok(values)
    }
}

impl From<&[WasmValue]> for svm_byte_array {
    fn from(values: &[WasmValue]) -> svm_byte_array {
        let nvalues = values.len();
        let capacity = 1 + nvalues * 9;

        let mut bytes = Vec::with_capacity(capacity);

        bytes.write_u8(0);

        for value in values.iter() {
            match value {
                WasmValue::I32(v) => {
                    bytes.write_u8(0);
                    bytes.write_u32::<BigEndian>(*v);
                }
                WasmValue::I64(v) => {
                    bytes.write_u8(1);
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
