use crate::types::{WasmType, WasmValue};

#[derive(Debug, Clone, PartialEq)]
pub struct WasmValueLayout {
    pub ty: WasmType,

    pub len: usize,
}

pub fn wasm_value_layout(wasm_value: &WasmValue) -> WasmValueLayout {
    match wasm_value {
        WasmValue::I32(v) => {
            let len = wasm_value_byte_length(*v as u64);
            debug_assert!(len <= 4);

            WasmValueLayout {
                ty: WasmType::I32,
                len,
            }
        }
        WasmValue::I64(v) => {
            let len = wasm_value_byte_length(*v);

            debug_assert!(len <= 8);

            WasmValueLayout {
                ty: WasmType::I64,
                len,
            }
        }
    }
}

fn wasm_value_byte_length(value: u64) -> usize {
    match value {
        0 => 0,
        0x01..=0xFF => 1,
        0x01_00..=0xFF_FF => 2,
        0x_01_00_00..=0xFF_FF_FF => 3,
        0x_01_00_00_00..=0xFF_FF_FF_FF => 4,
        0x_01_00_00_00_00..=0xFF_FF_FF_FF_FF => 5,
        0x_01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => 6,
        0x_01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => 7,
        0x_01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => 8,
    }
}
