use serde_json::Value;

use super::wasm_buffer;

///
/// Encodes a `spawn-app` json input into SVM `spawn-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`mem_ptr` parameter) and its length (`length` parameter).
///
/// Returns a pointer to a `transaction buffer` (8 bytes of `Header`)
///
/// See also: `alloc` and `free`
///
pub fn encode_spawn_app(ptr: usize) -> usize {
    let buf: &[u8] = wasm_buffer(ptr);

    let json: Value = serde_json::from_slice(buf).unwrap();

    // let template: Result<AppTemplate, String> = AppTemplate::try_from(json);

    0
}
