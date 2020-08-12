use svm_nibble::{nib, NibbleWriter};
use svm_types::WasmValue;

use crate::wasm::wasm_value_layout;

/// Encodes the wasm value.
/// Important: The wasm value layout isn't encoded under this method.
///
/// The reason for not encoding here the layout is to allow decoupling.
/// There are encoders that encode a vector of `WasmValueLayout` consecutively before encoding the values.
/// See: [`encode_func_args`][encode_func_args].
///
/// On other cases, encoders will encode one or more `WasmValue`s as `(WasmValueLayout, WasmValue)` pairs.
/// See: [`encode_gas_used`][encode_gas_used].
///
/// [encode_func_args]: ../func_args/encoder.rs
/// [encode_gas_used]: ../gas/encoder.rs
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
