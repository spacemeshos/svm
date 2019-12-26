use std::convert::TryFrom;

use wasmer_runtime_c_api::value::{wasmer_value, wasmer_value_t, wasmer_value_tag};

#[repr(C)]
pub enum svm_value_type {
    I32 = 1,
    I64 = 2,
}

#[allow(non_snake_case)]
#[repr(C)]
pub union svm_value {
    pub I32: i32,
    pub I64: i64,
}

#[repr(C)]
pub struct svm_value_t {
    pub ty: svm_value_type,
    pub value: svm_value,
}

pub enum ConvertValueError {
    NotSupportedType,
}

impl TryFrom<wasmer_value_tag> for svm_value_type {
    type Error = ConvertValueError;

    fn try_from(tag: wasmer_value_tag) -> Result<Self, Self::Error> {
        match tag {
            wasmer_value_tag::WASM_I32 => Ok(svm_value_type::I32),
            wasmer_value_tag::WASM_I64 => Ok(svm_value_type::I64),
            _ => Err(ConvertValueError::NotSupportedType),
        }
    }
}

impl TryFrom<wasmer_value> for svm_value {
    type Error = ConvertValueError;

    fn try_from(value: wasmer_value) -> Result<Self, Self::Error> {
        unsafe {
            match value {
                wasmer_value { I32: v } => Ok(svm_value { I32: v }),
                wasmer_value { I64: v } => Ok(svm_value { I64: v }),
                _ => Err(ConvertValueError::NotSupportedType),
            }
        }
    }
}

impl TryFrom<wasmer_value_t> for svm_value_t {
    type Error = ConvertValueError;

    fn try_from(value: wasmer_value_t) -> Result<Self, Self::Error> {
        let ty = svm_value_type::try_from(value.tag)?;
        let value = svm_value::try_from(value.value)?;

        Ok(svm_value_t { ty, value })
    }
}
