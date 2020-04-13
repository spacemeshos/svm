use std::convert::TryFrom;

use svm_app::types::WasmValue;

/// FFI representation for `SVM` value array
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct svm_value_array {
    /// Pointer to the first `svm_value`
    pub values: *const svm_value,

    /// Number or values
    pub length: u64,
}

/// Converting a `&[WasmValue]` into `svm_value_array`.
/// The `svm_value_array` should be released by manually.
///
/// ```rust
/// use std::convert::TryFrom;
///
/// use svm_app::types::WasmValue;
/// use svm_runtime_c_api::{svm_value, svm_value_array, svm_value_type};
///
/// let values = vec![WasmValue::I32(10), WasmValue::I64(20)];
/// let values: svm_value_array = values.into();
/// assert_eq!(values.length, 2);
///
/// let slice: &[svm_value] = unsafe { std::slice::from_raw_parts(values.values, 2) };
/// assert_eq!(slice[0], svm_value { ty: svm_value_type::SVM_I32, i32_val: 10, i64_val: 0 });
/// assert_eq!(slice[1], svm_value { ty: svm_value_type::SVM_I64, i32_val: 0,  i64_val: 20 });
/// ```
///
impl From<&[WasmValue]> for svm_value_array {
    fn from(values: &[WasmValue]) -> Self {
        let values: Vec<svm_value> = values.iter().map(|v| v.into()).collect();

        let (ptr, len, _cap) = values.into_raw_parts();

        Self {
            values: ptr,
            length: len as u64,
        }
    }
}

/// Converting a `Vec<WasmValue>` into `svm_value_array`.
/// The `svm_value_array` should be released by manually.
///
impl From<Vec<WasmValue>> for svm_value_array {
    #[inline]
    fn from(values: Vec<WasmValue>) -> Self {
        (&values[..]).into()
    }
}

/// FFI representation for `SVM` value
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct svm_value {
    /// Wasm integer type
    pub ty: svm_value_type,

    /// I32 value
    pub i32_val: u32,

    /// I64 value
    pub i64_val: u64,
}

/// Builds `svm_value` out of a `&WasmValue`.
///
/// # Example
///
/// ```rust
/// use svm_app::types::WasmValue;
/// use svm_runtime_c_api::{svm_value, svm_value_type};
///
/// let wasm_val: svm_value = WasmValue::I32(10).into();
/// assert_eq!(wasm_val, svm_value { ty: svm_value_type::SVM_I32, i32_val: 10, i64_val: 0 });
///
/// let wasm_val: svm_value = WasmValue::I64(20).into();
/// assert_eq!(wasm_val, svm_value { ty: svm_value_type::SVM_I64, i32_val: 0, i64_val: 20 });
/// ```
///
impl From<&WasmValue> for svm_value {
    fn from(val: &WasmValue) -> Self {
        match *val {
            WasmValue::I32(v) => Self {
                ty: svm_value_type::SVM_I32,
                i32_val: v,
                i64_val: 0,
            },
            WasmValue::I64(v) => Self {
                ty: svm_value_type::SVM_I64,
                i64_val: v,
                i32_val: 0,
            },
        }
    }
}

/// Builds `svm_value` out of a `WasmValue`.
impl From<WasmValue> for svm_value {
    #[inline]
    fn from(val: WasmValue) -> Self {
        (&val).into()
    }
}

/// FFI representation for `SVM` value type
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub enum svm_value_type {
    #[doc(hidden)]
    SVM_I32 = 1,

    #[doc(hidden)]
    SVM_I64 = 2,
}

///
/// # Example
///
/// ```rust
/// use std::convert::TryFrom;
/// use svm_runtime_c_api::svm_value_type;
///
/// let i32_type = svm_value_type::try_from(1);
/// let i64_type = svm_value_type::try_from(2);
/// let invalid_type = svm_value_type::try_from(3);
///
/// assert_eq!(svm_value_type::SVM_I32, i32_type.unwrap());
/// assert_eq!(svm_value_type::SVM_I64, i64_type.unwrap());
///
/// assert_eq!(Err("Invalid raw SVM value type: `3`".to_string()), invalid_type);
/// ```
///
impl TryFrom<u8> for svm_value_type {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            1 => Ok(svm_value_type::SVM_I32),
            2 => Ok(svm_value_type::SVM_I64),
            _ => Err(format!("Invalid raw SVM value type: `{}`", byte)),
        }
    }
}

/// FFI representation for an array of `svm_value_type`
#[allow(non_snake_case, non_camel_case_types)]
#[repr(C)]
pub struct svm_value_type_array {
    /// A raw pointer to beginning of array
    pub types: *const svm_value_type,

    /// Number of `types`
    pub length: u32,
}

///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::{svm_value_type, svm_value_type_array};
///
/// let type1 = svm_value_type::SVM_I32;
/// let type2 = svm_value_type::SVM_I64;
/// let types = vec![type1, type2];
/// let array: svm_value_type_array = (&types).into();
///
/// let vec: Vec<svm_value_type> = array.into();
/// assert_eq!(type1, vec[0]);
/// assert_eq!(type2, vec[1]);
/// assert_eq!(2, vec.len());
/// ```
///
impl From<svm_value_type_array> for Vec<svm_value_type> {
    fn from(array: svm_value_type_array) -> Self {
        let slice = unsafe { std::slice::from_raw_parts(array.types, array.length as usize) };

        slice.to_vec()
    }
}

///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::{svm_value_type, svm_value_type_array};
///
/// let type1 = svm_value_type::SVM_I32;
/// let type2 = svm_value_type::SVM_I64;
/// let types = vec![type1, type2];
///
/// let array: svm_value_type_array = (&types).into();
///
/// assert_eq!(2, array.length);
/// assert_eq!(types.as_ptr(), array.types);
/// ```
///
impl From<&Vec<svm_value_type>> for svm_value_type_array {
    fn from(vec: &Vec<svm_value_type>) -> Self {
        let length = vec.len() as u32;
        let types = vec.as_ptr();

        svm_value_type_array { types, length }
    }
}

impl From<Vec<svm_value_type>> for svm_value_type_array {
    fn from(vec: Vec<svm_value_type>) -> Self {
        (&vec).into()
    }
}
