use std::convert::TryFrom;

use svm_runtime::value::Value;

/// FFI representation for `SVM` value type
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum svm_value_type {
    #[doc(hidden)]
    SVM_I32 = 1,

    #[doc(hidden)]
    SVM_I64 = 2,
}

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

    /// Array number of items
    pub types_len: u32,
}

/// FFI representation for `SVM` value.
#[allow(non_snake_case, non_camel_case_types)]
#[repr(C)]
pub union svm_value {
    #[doc(hidden)]
    pub I32: i32,

    #[doc(hidden)]
    pub I64: i64,
}

/// FFI representation for `SVM` value + type.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_value_t {
    #[doc(hidden)]
    pub ty: svm_value_type,

    #[doc(hidden)]
    pub value: svm_value,
}

impl From<&Value> for svm_value_t {
    fn from(other: &Value) -> Self {
        match *other {
            Value::I32(v) => svm_value_t {
                ty: svm_value_type::SVM_I32,
                value: svm_value { I32: v },
            },
            Value::I64(v) => svm_value_t {
                ty: svm_value_type::SVM_I64,
                value: svm_value { I64: v },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svm_value_t_from_value_i32() {
        let value = Value::I32(10);
        let raw_value = svm_value_t::from(&value);
        assert_eq!(svm_value_type::SVM_I32, raw_value.ty);

        unsafe {
            let svm_value { I32: v } = raw_value.value;
            assert_eq!(10, v);
        }
    }

    #[test]
    fn svm_value_t_from_value_i64() {
        let value = Value::I64(10);
        let raw_value = svm_value_t::from(&value);
        assert_eq!(svm_value_type::SVM_I64, raw_value.ty);

        unsafe {
            let svm_value { I64: v } = raw_value.value;
            assert_eq!(10, v);
        }
    }
}
