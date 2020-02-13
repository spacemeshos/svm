use super::super::Nibble;

use crate::types::WasmType;

#[derive(Debug, Clone, PartialEq)]
pub struct WasmValueLayout {
    pub ty: WasmType,

    pub len: usize,
}

impl From<Nibble> for WasmValueLayout {
    fn from(nibble: Nibble) -> Self {
        match nibble.inner() {
            // 32-bit args layouts:
            0b_0000_0000 => Self {
                ty: WasmType::I32,
                len: 0,
            },
            0b_0000_0001 => Self {
                ty: WasmType::I32,
                len: 1,
            },
            0b_0000_0010 => Self {
                ty: WasmType::I32,
                len: 2,
            },
            0b_0000_0011 => Self {
                ty: WasmType::I32,
                len: 3,
            },
            0b_0000_0100 => Self {
                ty: WasmType::I32,
                len: 4,
            },
            //
            // 64-bit args layouts:
            0b_0000_0101 => Self {
                ty: WasmType::I64,
                len: 0,
            },
            0b_0000_1000 => Self {
                ty: WasmType::I64,
                len: 1,
            },
            0b_0000_1001 => Self {
                ty: WasmType::I64,
                len: 2,
            },
            0b_0000_1010 => Self {
                ty: WasmType::I64,
                len: 3,
            },
            0b_0000_1011 => Self {
                ty: WasmType::I64,
                len: 4,
            },
            0b_0000_1100 => Self {
                ty: WasmType::I64,
                len: 5,
            },
            0b_0000_1101 => Self {
                ty: WasmType::I64,
                len: 6,
            },
            0b_0000_1110 => Self {
                ty: WasmType::I64,
                len: 7,
            },
            0b_0000_1111 => Self {
                ty: WasmType::I64,
                len: 8,
            },
            _ => unreachable!(),
        }
    }
}
