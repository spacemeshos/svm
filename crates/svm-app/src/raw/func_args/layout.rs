use super::super::Nibble;

use crate::{nib, types::WasmType};

#[derive(Debug, Clone, PartialEq)]
pub struct WasmValueLayout {
    pub ty: WasmType,

    pub len: usize,
}

// special-cases
pub const NO_MORE: u8 = 0b_0000_0110;
pub const DO_SKIP: u8 = 0b_0000_0111;

// i32-layout
pub const I32_0B: u8 = 0b_0000_0000;
pub const I32_1B: u8 = 0b_0000_0001;
pub const I32_2B: u8 = 0b_0000_0010;
pub const I32_3B: u8 = 0b_0000_0011;
pub const I32_4B: u8 = 0b_0000_0100;

// i64-layout
pub const I64_0B: u8 = 0b_0000_0101;
pub const I64_1B: u8 = 0b_0000_1000;
pub const I64_2B: u8 = 0b_0000_1001;
pub const I64_3B: u8 = 0b_0000_1010;
pub const I64_4B: u8 = 0b_0000_1011;
pub const I64_5B: u8 = 0b_0000_1100;
pub const I64_6B: u8 = 0b_0000_1101;
pub const I64_7B: u8 = 0b_0000_1110;
pub const I64_8B: u8 = 0b_0000_1111;

impl Into<Nibble> for &WasmValueLayout {
    fn into(self) -> Nibble {
        let byte = {
            match (self.ty, self.len) {
                // i32
                (WasmType::I32, 0) => I32_0B,
                (WasmType::I32, 1) => I32_1B,
                (WasmType::I32, 2) => I32_2B,
                (WasmType::I32, 3) => I32_3B,
                (WasmType::I32, 4) => I32_4B,
                //
                // i64
                (WasmType::I64, 0) => I64_0B,
                (WasmType::I64, 1) => I64_1B,
                (WasmType::I64, 2) => I64_2B,
                (WasmType::I64, 3) => I64_3B,
                (WasmType::I64, 4) => I64_4B,
                (WasmType::I64, 5) => I64_5B,
                (WasmType::I64, 6) => I64_6B,
                (WasmType::I64, 7) => I64_7B,
                (WasmType::I64, 8) => I64_8B,
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
            I32_0B => Self {
                ty: WasmType::I32,
                len: 0,
            },
            I32_1B => Self {
                ty: WasmType::I32,
                len: 1,
            },
            I32_2B => Self {
                ty: WasmType::I32,
                len: 2,
            },
            I32_3B => Self {
                ty: WasmType::I32,
                len: 3,
            },
            I32_4B => Self {
                ty: WasmType::I32,
                len: 4,
            },
            //
            // 64-bit args layouts:
            I64_0B => Self {
                ty: WasmType::I64,
                len: 0,
            },
            I64_1B => Self {
                ty: WasmType::I64,
                len: 1,
            },
            I64_2B => Self {
                ty: WasmType::I64,
                len: 2,
            },
            I64_3B => Self {
                ty: WasmType::I64,
                len: 3,
            },
            I64_4B => Self {
                ty: WasmType::I64,
                len: 4,
            },
            I64_5B => Self {
                ty: WasmType::I64,
                len: 5,
            },
            I64_6B => Self {
                ty: WasmType::I64,
                len: 6,
            },
            I64_7B => Self {
                ty: WasmType::I64,
                len: 7,
            },
            I64_8B => Self {
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
