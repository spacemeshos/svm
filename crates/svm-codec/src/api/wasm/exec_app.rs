use serde_json::Value;

use svm_types::AppTransaction;

use super::{alloc, into_wasm_buffer, wasm_buffer};
use crate::{api, NibbleWriter};

///
/// Encodes a `exec-app` json input into SVM `exec-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_exec_app(ptr: usize) -> usize {
    let slice = wasm_buffer(ptr);
    let json: Value = serde_json::from_slice(slice).unwrap();
    let tx = api::json::exec_app(&json);
    let tx = tx.unwrap();

    let mut w = NibbleWriter::new();
    crate::encode_exec_app(&tx, &mut w);

    let bytes = w.into_bytes();
    into_wasm_buffer(bytes)
}
