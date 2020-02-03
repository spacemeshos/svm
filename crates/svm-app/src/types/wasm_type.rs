use std::convert::TryFrom;

/// `WasmType` - Wasm primitive type.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmType {
    /// Represents a 4-byte integer argument.
    I32,

    /// Represents a 8-byte integer argument.
    I64,
}

/// Converts `WasmType` to its numeric representation
impl Into<u8> for WasmType {
    fn into(self) -> u8 {
        match self {
            WasmType::I32 => 1,
            WasmType::I64 => 2,
        }
    }
}

/// Wasm function arguments error
pub enum WasmConvertTypeError {
    /// Unsupported type
    UnsupportedType(u8),
}

/// Converts `WasmType` to its numeric representation
impl TryFrom<u8> for WasmType {
    type Error = WasmConvertTypeError;

    fn try_from(value: u8) -> Result<WasmType, WasmConvertTypeError> {
        match value {
            1 => Ok(WasmType::I32),
            2 => Ok(WasmType::I64),
            _ => Err(WasmConvertTypeError::UnsupportedType(value)),
        }
    }
}
