use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

use svm_app::types::WasmType;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::svm_byte_array;

// TODO: delete and stay with `TryFrom`
impl From<svm_byte_array> for Vec<WasmType> {
    fn from(bytes: svm_byte_array) -> Self {
        let slice: &[u8] =
            unsafe { std::slice::from_raw_parts(bytes.bytes, bytes.length as usize) };

        let length = slice.len();
        if length == 0 {
            // ErrorKind::InvalidInput.into()
            panic!("invalid input...")
        }

        let ntypes = slice[0];
        let mut types = Vec::with_capacity(ntypes as usize);
        let mut cursor = Cursor::new(&slice[1..]);

        for _ in 0..ntypes {
            let raw_ty = cursor.read_u8().expect("invalid input...");
            let ty = WasmType::try_from(raw_ty).unwrap();

            types.push(ty);
        }

        types
    }
}

impl From<&[WasmType]> for svm_byte_array {
    fn from(types: &[WasmType]) -> svm_byte_array {
        let ntypes = types.len();
        let capacity = 1 + ntypes;

        let mut bytes = Vec::with_capacity(capacity);

        bytes.write_u8(0);

        for ty in types.iter() {
            let ty = ty.into();
            bytes.write_u8(ty);
        }

        bytes.into()
    }
}
