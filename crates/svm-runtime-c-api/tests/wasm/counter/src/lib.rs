#![feature(vec_into_raw_parts)]

use svm_sdk::traits::Encoder;
use svm_sdk::CallData;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const VAR_ID: u32 = 0;

#[link(wasm_import_module = "svm")]
extern "C" {
    fn svm_calldata_offset() -> u32;

    fn svm_calldata_len() -> u32;

    fn svm_set_returndata(offset: u32, length: u32);

    fn svm_get32(var_id: u32) -> u32;

    fn svm_set32(var_id: u32, value: u32);
}

#[link(wasm_import_module = "host")]
extern "C" {
    fn counter_mul(var_id: u32, mul: u32) -> u32;
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
    let ptr = svm_sdk::alloc(size as usize);

    ptr.offset() as i32
}

#[no_mangle]
pub extern "C" fn initialize() {
    let bytes = get_calldata();
    let mut calldata = CallData::new(bytes);

    let initial: u32 = calldata.next_1();

    unsafe {
        svm_set32(VAR_ID, initial);
    }
}

#[no_mangle]
pub extern "C" fn add_and_mul() {
    let calldata = get_calldata();

    let mut calldata = CallData::new(calldata);

    let add: u32 = calldata.next_1();
    let mul: u32 = calldata.next_1();

    unsafe {
        let a = svm_get32(VAR_ID);
        let b = a + add;
        svm_set32(VAR_ID, b);

        let c = counter_mul(VAR_ID, mul);
        svm_set32(VAR_ID, c);

        let mut buf = Vec::new();
        let results = vec![a, b, c];
        results.encode(&mut buf);

        let (ptr, len, _cap) = buf.into_raw_parts();

        svm_set_returndata(ptr as u32, len as u32);
    }
}
