/// Wasm Integer.
#[derive(Clone, PartialEq, Debug)]
pub enum WasmValue {
    /// A 32-bit integer.
    I32(u32),

    /// A 64-bit integer.
    I64(u64),
}
