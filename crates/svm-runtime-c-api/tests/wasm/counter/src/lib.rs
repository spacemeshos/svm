use svm_abi_decoder::CallData;
use svm_sdk;

const VAR_ID: u32 = 0;

#[link(wasm_import_module = "svm")]
extern "C" {
    fn svm_calldata_offset() -> i32;

    fn svm_calldata_len() -> i32;

    fn svm_get32(var_id: u32) -> u32;

    fn svm_set32(var_id: u32, value: u32);
}

fn get_calldata() -> &'static [u8] {
    unsafe {
        let ptr = svm_calldata_offset();
        let len = svm_calldata_len();

        core::slice::from_raw_parts(ptr as *const u8, len as usize)
    }
}

#[no_mangle]
pub extern "C" fn svm_alloc(size: i32) -> i32 {
    svm_sdk::memory::alloc(size as usize) as i32
}

#[no_mangle]
pub extern "C" fn initialize() {
    let calldata = get_calldata();

    let mut calldata = CallData::new(calldata);

    let initial: u32 = calldata.next_1();

    unsafe {
        svm_set32(VAR_ID, initial);
    }
}

#[no_mangle]
pub extern "C" fn add() {
    let calldata = get_calldata();

    let mut calldata = CallData::new(calldata);

    let addition: u32 = calldata.next_1();

    unsafe {
        let old = svm_get32(VAR_ID);
        let new = old + addition;

        svm_set32(VAR_ID, new);
    }
}
