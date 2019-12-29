use wasmer_runtime_core::types::{Type as WasmerType, Value as WasmerValue};

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Value {
    I32(i32),
    I64(i64),
}

#[derive(Debug, PartialEq)]
pub enum ValueCastError {
    NotSupportedType(&'static str),
}

impl TryFrom<WasmerValue> for Value {
    type Error = ValueCastError;

    fn try_from(value: WasmerValue) -> Result<Self, Self::Error> {
        match value {
            WasmerValue::I32(v) => Ok(Value::I32(v)),
            WasmerValue::I64(v) => Ok(Value::I64(v)),
            WasmerValue::F32(_) => Err(ValueCastError::NotSupportedType("F32")),
            WasmerValue::F64(_) => Err(ValueCastError::NotSupportedType("F64")),
            WasmerValue::V128(_) => Err(ValueCastError::NotSupportedType("V128")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_try_from() {
        let wasmer_value = WasmerValue::I32(10);
        assert_eq!(Ok(Value::I32(10)), Value::try_from(wasmer_value));

        let wasmer_value = WasmerValue::I64(20);
        assert_eq!(Ok(Value::I64(20)), Value::try_from(wasmer_value));

        let wasmer_value = WasmerValue::F32(10.0);
        assert_eq!(
            Err(ValueCastError::NotSupportedType("F32")),
            Value::try_from(wasmer_value)
        );

        let wasmer_value = WasmerValue::F64(20.0);
        assert_eq!(
            Err(ValueCastError::NotSupportedType("F64")),
            Value::try_from(wasmer_value)
        );

        let wasmer_value = WasmerValue::V128(0);
        assert_eq!(
            Err(ValueCastError::NotSupportedType("V128")),
            Value::try_from(wasmer_value)
        );
    }
}
