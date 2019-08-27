use std::convert::TryFrom;

/// `WasmArgType` defines 4 types:
/// * I32 - Represents a 4-byte integer argument.
///
/// * I64 - Represents a 8-byte integer argument.
///
/// * Fixed - Represents a fixed-size array of bytes.
///   For example: `Address` is a 32 bytes fixed-array.
///
/// * Slice - Represents a blob of data, not known ahead.
///   For example: `String`
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmArgType {
    I32,
    I64,
    Fixed,
    Slice,
}

/// The actual value of `wasm` argument.
///
/// * I32 - A 4-byte integer.
///
/// * I64 - A 8-byte integer.
///
/// * Fixed - A tuple of `(WasmInt, Vec<u8>)`
///     * `WasmInt` - Stores the start offset in wasm linear-memory the copied fixed-array starts.
///     This value isn't part of the transaction raw bytes, but it's being infered
///     as part of initializing the wasm instance memory.
///
///     * `Vec<u8>` - The bytes of the fixed-array.
///
/// * Slice - A tuple of `(WasmInt, WasmInt, Vec<u8>)`
///     * `WasmInt` (the left one) - Stores the start offset in wasm linear-memory the copied fixed-array starts.
///     This value isn't part of the transaction raw bytes, but it's being infered
///     as part of initializing the wasm instance memory.
///
///     * `WasmInt` (the right one) - Stores the length of the slice.
///
///     * `Vec<u8>` - The bytes of the slice.
#[derive(Clone, PartialEq, Debug)]
pub enum WasmArgValue {
    I32(u32),
    I64(u64),
    Fixed(WasmInt, Vec<u8>),
    Slice(WasmInt, WasmInt, Vec<u8>),
}

/// Represents a `wasm` Integer
/// * I32 - Represents a 4-byte integer argument.
/// * I64 - Represents a 8-byte integer argument.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmInt {
    I32(i32),
    I64(i64),
}

/// Converts `WasmArgType` to its numeric representation
impl Into<u8> for WasmArgType {
    fn into(self) -> u8 {
        match self {
            WasmArgType::I32 => 0,
            WasmArgType::I64 => 1,
            WasmArgType::Fixed => 2,
            WasmArgType::Slice => 3,
        }
    }
}

pub enum WasmArgTypeError {
    UnsupportedType(u8),
}

/// Converts `WasmArgType` to its numeric representation
impl TryFrom<u8> for WasmArgType {
    type Error = WasmArgTypeError;

    fn try_from(value: u8) -> Result<WasmArgType, WasmArgTypeError> {
        match value {
            0 => Ok(WasmArgType::I32),
            1 => Ok(WasmArgType::I64),
            2 => Ok(WasmArgType::Fixed),
            3 => Ok(WasmArgType::Slice),
            _ => Err(WasmArgTypeError::UnsupportedType(value)),
        }
    }
}
