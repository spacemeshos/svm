use std::convert::TryFrom;

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
