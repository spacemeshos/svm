#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmArgValue {
    I32(u32),
    I64(u64),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmArgType {
    I32,
    I64,
}

impl Into<u8> for WasmArgType {
    fn into(self) -> u8 {
        match self {
            WasmArgType::I32 => 0,
            WasmArgType::I64 => 1,
        }
    }
}
