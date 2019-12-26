use wasmer_runtime_core::types::{Type as WasmerType, Value as WasmerValue};

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Value {
    I32(i32),
    I64(i64),
}

pub enum ValueCastError {
    NotSupportType(&'static str),
}

impl TryFrom<WasmerValue> for Value {
    type Error = ValueCastError;

    fn try_from(value: WasmerValue) -> Result<Self, Self::Error> {
        match value {
            WasmerValue::I32(v) => Ok(Value::I32(v)),
            WasmerValue::I64(v) => Ok(Value::I64(v)),
            WasmerValue::F32(_) => Err(ValueCastError::NotSupportType("F32")),
            WasmerValue::F64(_) => Err(ValueCastError::NotSupportType("F64")),
            WasmerValue::V128(_) => Err(ValueCastError::NotSupportType("V128")),
        }
    }
}
