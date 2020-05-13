use std::convert::TryFrom;
use std::io::{self, Cursor, Error, ErrorKind};

use svm_app::types::WasmType;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::svm_byte_array;

impl TryFrom<svm_byte_array> for Vec<WasmType> {
    type Error = io::Error;

    fn try_from(bytes: svm_byte_array) -> Result<Self, Self::Error> {
        let slice: &[u8] =
            unsafe { std::slice::from_raw_parts(bytes.bytes, bytes.length as usize) };

        let length = slice.len();
        if length == 0 {
            return Err(ErrorKind::InvalidInput.into());
        }

        let ntypes = slice[0];
        let mut types = Vec::with_capacity(ntypes as usize);
        let mut cursor = Cursor::new(&slice[1..]);

        for _ in 0..ntypes {
            let raw_ty = cursor.read_u8()?;
            let ty = WasmType::try_from(raw_ty).unwrap();

            types.push(ty);
        }

        Ok(types)
    }
}

impl From<&[WasmType]> for svm_byte_array {
    fn from(types: &[WasmType]) -> svm_byte_array {
        let ntypes = types.len();

        assert!(ntypes <= std::u8::MAX as usize);

        let capacity = 1 + ntypes;

        let mut bytes = Vec::with_capacity(capacity);

        bytes.write_u8(ntypes as u8);

        for ty in types.iter() {
            let ty = ty.into();
            bytes.write_u8(ty);
        }

        bytes.into()
    }
}

impl From<Vec<WasmType>> for svm_byte_array {
    #[inline]
    fn from(types: Vec<WasmType>) -> svm_byte_array {
        (&types[..]).into()
    }
}

impl From<&Vec<WasmType>> for svm_byte_array {
    #[inline]
    fn from(types: &Vec<WasmType>) -> svm_byte_array {
        (&types[..]).into()
    }
}
