use crate::WasmType;

/// Wasm Integer.
#[derive(Clone, PartialEq, Debug)]
pub enum WasmValue {
    /// A 32-bit integer.
    I32(u32),

    /// A 64-bit integer.
    I64(u64),
}

impl WasmValue {
    /// Returns the `WasmType` of self.
    #[inline]
    pub fn ty(&self) -> WasmType {
        match self {
            WasmValue::I32(..) => WasmType::I32,
            WasmValue::I64(..) => WasmType::I64,
        }
    }

    #[inline]
    pub fn as_i32(&self) -> Option<u32> {
        match self {
            WasmValue::I32(v) => Some(*v),
            WasmValue::I64(v) => None,
        }
    }

    #[inline]
    pub fn as_i64(&self) -> Option<u64> {
        match self {
            WasmValue::I32(v) => None,
            WasmValue::I64(v) => Some(*v),
        }
    }
}

/// Returns the `WasmValue` internal integer as `u64`.
///
/// # Example
///
/// ```
/// use svm_types::WasmValue;
///
/// let n: u64 = WasmValue::I32(10).into();
/// let m: u64 = WasmValue::I64(20).into();
///
/// assert_eq!(n, 10u64);
/// assert_eq!(m, 20u64);
/// ```
impl From<&WasmValue> for u64 {
    #[inline]
    fn from(value: &WasmValue) -> u64 {
        match value {
            WasmValue::I32(v) => *v as u64,
            WasmValue::I64(v) => *v,
        }
    }
}

impl From<WasmValue> for u64 {
    #[inline]
    fn from(value: WasmValue) -> u64 {
        (&value).into()
    }
}

/// Given a tuple of `WasmType` and `u64` - returns `WasmValue`.
///
/// # Example
///
/// ```
/// use svm_types::{WasmType, WasmValue};
///
/// assert_eq!(WasmValue::I32(10), (WasmType::I32, 10u64).into());
/// assert_eq!(WasmValue::I64(20), (WasmType::I64, 20u64).into());
/// ```
impl From<(WasmType, u64)> for WasmValue {
    #[inline]
    fn from((ty, value): (WasmType, u64)) -> WasmValue {
        match ty {
            WasmType::I32 => WasmValue::I32(value as u32),
            WasmType::I64 => WasmValue::I64(value),
        }
    }
}
