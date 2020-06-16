use serde_json::Value;

use super::allocate;

#[no_mangle]
pub unsafe extern "C" fn encode_spawn_app(mem_ptr: i32, length: i32) -> i32 {
    let slice = std::slice::from_raw_parts(mem_ptr as _, length as usize);
    let json: Value = serde_json::from_slice(slice).unwrap();

    // let template: Result<AppTemplate, String> = AppTemplate::try_from(json);

    0
}
