use svm_types::WasmValue;

use crate::{wasm, NibbleWriter};

/// Encodes the `gas_used` field as part of a `Receipt`.
#[allow(unused)]
pub fn encode_gas_used(gas_used: u64, w: &mut NibbleWriter) {
    let value = WasmValue::I64(gas_used);
    let layout = wasm::wasm_value_layout(&value);

    let nib = (&layout).into();
    w.write(&[nib]);

    wasm::encode_wasm_value(&value, w);
}
