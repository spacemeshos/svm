use std::convert::TryFrom;
use std::io::{self, Cursor};

use svm_types::WasmType;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::svm_byte_array;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TypeIdOrStr {
    TypeId(std::any::TypeId, &'static str),

    Str(&'static str),
}

///
/// This file contains the implementation of encoding & decoding of a `Vec<WasmType>` into `svm_byte_array`.
/// (and vice-versa).
///
/// This encoding (and decoding) functionality should be also implemented by any SVM clients (e.g: C, Go).
/// The design motivation is sticking with `svm_byte_array` as the mechanism for passing data between SVM client
/// to SVM (via the `SVM C-API`)
///
/// ### Encoding format:
///
/// * Then each WASM type is encoded as a `I32` or `I64` (SVM doesn't support Floats).
///   The encoding is implemented of `WasmType` to `u8` sits under the `svm-codec` crate.
///
/// Note: the number of types equals the buffer length (one byte per-type).
///
/// +-----------+--------+------------+
/// |  type #1  |  . . . |  type #N   |
/// |  (1 byte) |        |  (1 byte)  |
/// +-----------+--------+------------+
///

/// Converts `Vec<WasmType>` into `svm_byte_array`
///
/// # Example
///
/// ````
/// use std::io;
/// use std::convert::TryFrom;
///
/// use svm_types::WasmType;
/// use svm_ffi::svm_byte_array;
///
/// let types = vec![WasmType::I32, WasmType::I64, WasmType::I32];
/// let bytes: svm_byte_array = types.into();
///
/// let types: Result<Vec<WasmType>, io::Error> = Vec::try_from(bytes);
/// assert_eq!(types.unwrap(), vec![WasmType::I32, WasmType::I64, WasmType::I32]);
/// ````
///
impl From<&[WasmType]> for svm_byte_array {
    fn from(types: &[WasmType]) -> svm_byte_array {
        let ntypes = types.len();

        assert!(ntypes <= std::u8::MAX as usize);

        let mut bytes = Vec::with_capacity(ntypes);

        for ty in types.iter() {
            let ty = ty.into();
            bytes.write_u8(ty).unwrap();
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

        let ntypes = slice.len();

        let mut types = Vec::with_capacity(ntypes);
        let mut cursor = Cursor::new(slice);

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
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn empty_svm_byte_array_to_vec_types_errors() {
        let bytes = svm_byte_array {
            bytes: std::ptr::null(),
            length: 0,
            capacity: 0,
            type_id: 0,
        };

        let res: Result<Vec<WasmType>, io::Error> = Vec::try_from(bytes);
        assert_eq!(res.unwrap(), vec![]);
    }
}
