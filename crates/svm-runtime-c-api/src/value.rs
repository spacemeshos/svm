use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

use svm_app::types::{WasmType, WasmValue};

use crate::svm_byte_array;

// TODO: delete and stay with `TryFrom`
impl From<svm_byte_array> for Vec<WasmValue> {
    fn from(value: svm_byte_array) -> Self {
        todo!()
    }
}

// impl TryFrom<svm_byte_array> for Vec<WasmValue> {
//     type Error = Error;

//     fn try_from(bytes: svm_byte_array) -> Result<Self, Self::Error> {
//         let slice: &[u8] = std::slice::from_raw_parts(bytes.bytes, bytes.length as usize);

//         let length = slice.len();
//         if length == 0 {
//             return Err(ErrorKind::InvalidInput.into());
//         }

//         let nvalues: u8 = slice[0];

//         todo!()
//     }
// }
