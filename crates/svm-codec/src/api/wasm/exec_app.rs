use serde_json::Value;

use svm_types::AppTransaction;

use super::{alloc, wasm_buffer};
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
    0
}
