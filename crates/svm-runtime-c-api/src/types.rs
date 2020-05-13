use std::convert::TryFrom;
use std::io::{self, Cursor, Error, ErrorKind};

use svm_app::types::WasmType;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::svm_byte_array;

/// Converts `Vec<WasmType>` into `svm_byte_array`
///
/// # Examples
///
/// ````
/// use std::io;
/// use std::convert::TryFrom;
///
/// use svm_app::types::WasmType;
/// use svm_runtime_c_api::svm_byte_array;
///
/// let types = vec![WasmType::I32, WasmType::I64, WasmType::I32];
/// let bytes: svm_byte_array = types.into();
///
/// let types: Result<Vec<WasmType>, io::Error> = Vec::try_from(bytes);
/// assert_eq!(types.unwrap(), vec![WasmType::I32, WasmType::I64, WasmType::I32]);
/// ````
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec_types_to_svm_byte_array() {
        let bytes: svm_byte_array = Vec::<WasmType>::new().into();

        let slice: &[u8] = bytes.into();
        assert_eq!(slice, &[0]);
    }

    #[test]
    fn empty_svm_byte_array_to_vec_types_errors() {
        let bytes = svm_byte_array {
            bytes: std::ptr::null(),
            length: 0,
        };

        let res: Result<Vec<WasmType>, io::Error> = Vec::try_from(bytes);

        assert!(res.is_err());
    }

    #[test]
    fn svm_byte_array_to_vec_types_with_zero_items() {
        let bytes = svm_byte_array {
            bytes: vec![0].as_ptr(),
            length: 1,
        };

        let res: Result<Vec<WasmType>, io::Error> = Vec::try_from(bytes);
        assert_eq!(res.unwrap(), vec![]);
    }

    #[test]
    fn svm_byte_array_to_vec_types_with_missing_type_bytes() {
        let bytes = svm_byte_array {
            bytes: vec![1].as_ptr(),
            length: 1,
        };

        let res: Result<Vec<WasmType>, io::Error> = Vec::try_from(bytes);
        assert!(res.is_err());
    }
}
