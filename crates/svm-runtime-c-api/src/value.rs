use std::convert::TryFrom;
use svm_runtime::value::Value;

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum svm_value_type {
    I32 = 1,
    I64 = 2,
}

#[allow(non_snake_case, non_camel_case_types)]
#[repr(C)]
pub union svm_value {
    pub I32: i32,
    pub I64: i64,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_value_t {
    pub ty: svm_value_type,
    pub value: svm_value,
}

impl From<&Value> for svm_value_t {
    fn from(other: &Value) -> Self {
        match *other {
            Value::I32(v) => svm_value_t {
                ty: svm_value_type::I32,
                value: svm_value { I32: v },
            },
            Value::I64(v) => svm_value_t {
                ty: svm_value_type::I64,
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
        assert_eq!(svm_value_type::I32, raw_value.ty);

        unsafe {
            let svm_value { I32: v } = raw_value.value;
            assert_eq!(10, v);
        }
    }

    #[test]
    fn svm_value_t_from_value_i64() {
        let value = Value::I64(10);
        let raw_value = svm_value_t::from(&value);
        assert_eq!(svm_value_type::I64, raw_value.ty);

        unsafe {
            let svm_value { I64: v } = raw_value.value;
            assert_eq!(10, v);
        }
    }
}
