use serde_json::Value;

///
/// Encodes a `spawn-app` json input into SVM `spawn-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`mem_ptr` parameter) and its length (`length` parameter).
///
/// Returns sl
///
#[no_mangle]
pub unsafe extern "C" fn encode_spawn_app(mem_ptr: i32, length: i32) -> i32 {
    let slice = std::slice::from_raw_parts(mem_ptr as _, length as usize);
    let json: Value = serde_json::from_slice(slice).unwrap();

    // let template: Result<AppTemplate, String> = AppTemplate::try_from(json);

    0
}
