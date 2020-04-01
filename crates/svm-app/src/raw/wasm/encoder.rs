use crate::nib;
use crate::types::WasmValue;

use super::{super::NibbleWriter, wasm_value_layout, WasmValueLayout};

pub fn encode_wasm_value(wasm_value: &WasmValue, w: &mut NibbleWriter) {
    let layout = wasm_value_layout(wasm_value);

    let mut nibbles = Vec::with_capacity(layout.len);

    let mut val = match wasm_value {
        WasmValue::I32(v) => *v as u64,
        WasmValue::I64(v) => *v,
    };

    for _ in 0..layout.len {
        let byte = (val & 0xFF) as u8;
        let lnib = nib!((byte & 0xF0) >> 4);
        let rnib = nib!(byte & 0x0F);

        nibbles.push(rnib);
        nibbles.push(lnib);

        // rotate byte
        val >>= 8;
    }

    // since we've scanned `val` from `lsb` to `msb` order,
    // we need to reverse `nibbles` prior calling `w` with them.
    let nibbles: Vec<_> = nibbles.drain(..).rev().collect();

    w.write(&nibbles[..])
}
