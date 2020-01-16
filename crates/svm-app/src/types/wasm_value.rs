#[derive(Clone, PartialEq, Debug)]
pub enum WasmValue {
    /// A 32-bit signed integer.
    I32(i32),

    /// A 64-bit singed integer.
    I64(i64),
}
