use std::convert::TryFrom;

/// `WasmArgType` - wasm function argument type.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmArgType {
    /// Represents a 4-byte integer argument.
    I32,

    /// Represents a 8-byte integer argument.
    I64,
}

/// The actual value of a `wasm` argument.
///
///  The actual values of these `WasmArgType` **aren't** part of the execution-transaction raw data.
///  These values will be inferred as part of preparing the wasm instance for execution,
///  When initializing the wasm instance memory and passing the call arguments to the invoked function.
#[derive(Clone, PartialEq, Debug)]
pub enum WasmArgValue {
    /// A 4-byte integer.
    I32(u32),

    /// A 8-byte integer.
    I64(u64),
}

/// Converts `WasmArgType` to its numeric representation
impl Into<u8> for WasmArgType {
    fn into(self) -> u8 {
        match self {
            WasmArgType::I32 => 0,
            WasmArgType::I64 => 1,
        }
    }
}

/// Wasm function arguments error
pub enum WasmArgTypeError {
    /// Unsupported type
    UnsupportedType(u8),
}

/// Converts `WasmArgType` to its numeric representation
impl TryFrom<u8> for WasmArgType {
    type Error = WasmArgTypeError;

    fn try_from(value: u8) -> Result<WasmArgType, WasmArgTypeError> {
        match value {
            0 => Ok(WasmArgType::I32),
            1 => Ok(WasmArgType::I64),
            _ => Err(WasmArgTypeError::UnsupportedType(value)),
        }
    }
}
