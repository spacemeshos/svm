// #[derive(Clone, PartialEq, Debug)]
// pub enum WasmArgValue {
//     Bytes(Vec<u8>),
//     I32(u32),
//     I64(u64),
// }
//
// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum WasmArgType {
//     Register,
//     Param,
// }
//
// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum WasmArgStore {
//     Param(u8),
//     Register(u16, u16),
//     TempBuffer,
// }
//
// impl Into<u8> for WasmArgType {
//     fn into(self) -> u8 {
//         match self {
//             WasmArgType::Bytes => 0,
//             WasmArgType::I32 => 1,
//             WasmArgType::I64 => 2,
//         }
//     }
// }
