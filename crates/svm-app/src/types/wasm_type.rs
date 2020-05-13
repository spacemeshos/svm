use std::convert::TryFrom;

/// `WasmType` - Wasm primitive type.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmType {
    /// Represents a 4-byte integer argument.
    I32,

    /// Represents a 8-byte integer argument.
    I64,
}

/// Converts `WasmType` to its raw representation
///
/// ```
/// use svm_app::types::WasmType;
///
/// let ty: u8 = WasmType::I32.into();
/// assert_eq!(ty, 0u8);
///
/// let ty: u8 = WasmType::I64.into();
/// assert_eq!(ty, 1u8);
/// ```
impl From<&WasmType> for u8 {
    fn from(ty: &WasmType) -> u8 {
        match ty {
            WasmType::I32 => 0,
            WasmType::I64 => 1,
        }
    }
}

impl From<WasmType> for u8 {
    #[inline]
    fn from(ty: WasmType) -> u8 {
        (&ty).into()
    }
}

/// Wasm function arguments error
#[derive(Debug, PartialEq, Eq)]
pub enum WasmTypeError {
    /// Unsupported type
    UnsupportedType(u8),
}

/// Converts `WasmType` to its raw representation
///
/// ```
/// use std::convert::TryFrom;
/// use svm_app::types::{WasmType, WasmTypeError};
///
/// let ty = WasmType::try_from(0u8).unwrap();
/// assert_eq!(ty, WasmType::I32);
///
/// let ty = WasmType::try_from(1u8).unwrap();
/// assert_eq!(ty, WasmType::I64);
///
/// let err = WasmType::try_from(2u8).err().unwrap();
/// assert_eq!(err, WasmTypeError::UnsupportedType(2u8));
/// ```
impl TryFrom<u8> for WasmType {
    type Error = WasmTypeError;

    fn try_from(value: u8) -> Result<WasmType, WasmTypeError> {
        match value {
            0 => Ok(WasmType::I32),
            1 => Ok(WasmType::I64),
            _ => Err(WasmTypeError::UnsupportedType(value)),
        }
    }
}
