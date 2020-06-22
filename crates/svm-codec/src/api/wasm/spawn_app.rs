use serde_json::{self as json, Value};

use svm_types::SpawnApp;

use super::{error::into_error_buffer, wasm_buffer_data};
use crate::{api, NibbleWriter};

/// Encodes a `spawn-app` json input into SVM `spawn-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
pub fn encode_spawn_app(ptr: usize) -> usize {
    let bytes = wasm_buffer_data(ptr);
    let json: json::Result<Value> = serde_json::from_slice(bytes);

    match json {
        Ok(json) => todo!("..."),
        Err(err) => into_error_buffer(err),
    }
}
