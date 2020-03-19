use crate::nib;

use crate::types::{WasmType, WasmValue};

use super::super::NibbleWriter;
use super::{WasmValueLayout, NO_MORE};

pub fn encode_func_args(args: &[WasmValue], w: &mut NibbleWriter) {
    encode_func_values(args, w)
}

pub fn encode_func_rets(rets: &[WasmValue], w: &mut NibbleWriter) {
    encode_func_values(rets, w)
}

fn encode_func_values(values: &[WasmValue], w: &mut NibbleWriter) {
    let mut layouts = Vec::with_capacity(values.len());

    for val in values.iter() {
        let layout = wasm_value_layout(val);
        let nib = (&layout).into();

        layouts.push(layout);
        w.write(&[nib]);
    }

    // output `no more func values layouts` marker.
    let no_more_nib = nib!(NO_MORE);
    w.write(&[no_more_nib]);

    // write the func values
    for (i, val) in values.iter().enumerate() {
        let layout = &layouts[i];

        encode_func_wasm_val(val, layout, w);
    }
}

fn wasm_value_layout(value: &WasmValue) -> WasmValueLayout {
    match value {
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

fn encode_func_wasm_val(arg: &WasmValue, layout: &WasmValueLayout, w: &mut NibbleWriter) {
    let mut nibbles = Vec::with_capacity(layout.len);

    let mut val = match arg {
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
