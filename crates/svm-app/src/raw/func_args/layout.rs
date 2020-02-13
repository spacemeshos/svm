use super::super::Nibble;

use crate::{nib, types::WasmType};

#[derive(Debug, Clone, PartialEq)]
pub struct WasmValueLayout {
    pub ty: WasmType,

    pub len: usize,
}

impl Into<Nibble> for &WasmValueLayout {
    fn into(self) -> Nibble {
        let byte = {
            match (self.ty, self.len) {
                // i32
                (WasmType::I32, 0) => 0b_0000,
                (WasmType::I32, 1) => 0b_0001,
                (WasmType::I32, 2) => 0b_0010,
                (WasmType::I32, 3) => 0b_0011,
                (WasmType::I32, 4) => 0b_0100,
                //
                // i64
                (WasmType::I64, 0) => 0b_0101,
                (WasmType::I64, 1) => 0b_1000,
                (WasmType::I64, 2) => 0b_1001,
                (WasmType::I64, 3) => 0b_1010,
                (WasmType::I64, 4) => 0b_1011,
                (WasmType::I64, 5) => 0b_1100,
                (WasmType::I64, 6) => 0b_1101,
                (WasmType::I64, 7) => 0b_1110,
                (WasmType::I64, 8) => 0b_1111,
                _ => unreachable!(),
            }
        };

        nib!(byte)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_layout_to_nib(ty: WasmType, len: usize) {
        let layout = WasmValueLayout { ty, len };

        let nib: Nibble = (&layout).into();

        assert_eq!(layout, nib.into());
    }

    #[test]
    fn wasm_value_layout_to_nibble_and_back() {
        for len in 0..5 {
            assert_layout_to_nib(WasmType::I32, len);
        }

        for len in 0..9 {
            assert_layout_to_nib(WasmType::I64, len);
        }
    }
}
