use crate::nib;
use crate::types::WasmValue;

use super::super::{
    wasm::{encode_wasm_value, wasm_value_layout, NO_MORE},
    NibbleWriter,
};

/// Encodes func args
pub fn encode_func_args(args: &[WasmValue], w: &mut NibbleWriter) {
    encode_func_values(args, w)
}

/// Encodes func returns
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
    for val in values.iter() {
        encode_wasm_value(val, w);
    }
}
