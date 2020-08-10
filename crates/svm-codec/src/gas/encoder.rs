use svm_nibble::NibbleWriter;
use svm_types::{gas::MaybeGas, WasmValue};

use crate::wasm;

/// Encodes the `gas_used` field as part of a `Receipt`.
#[allow(unused)]
pub fn encode_gas_used(gas_used: &MaybeGas, w: &mut NibbleWriter) {
    let gas_used = gas_used.unwrap_or(0);
    let value = WasmValue::I64(gas_used);
    let layout = wasm::wasm_value_layout(&value);

    let nib = (&layout).into();
    w.write(&[nib]);

    wasm::encode_wasm_value(&value, w);
}
