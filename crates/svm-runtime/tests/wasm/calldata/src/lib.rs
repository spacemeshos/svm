#![no_std]

use svm_abi_decoder::{Cursor, Decoder};
use svm_sdk::value::Address;

const VAR_ID: i32 = 0;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[link(wasm_import_module = "svm")]
extern "C" {
    fn calldata_ptr() -> i32;

    fn calldata_len() -> i32;

    fn load160(var_id: i32, mem_idx: i32, mem_ptr: i32);

    fn store160(mem_idx: i32, mem_ptr: i32, var_id: i32);
}

fn get_calldata() -> &'static [u8] {
    unsafe {
        let ptr = calldata_ptr();
        let len = calldata_len();

        core::slice::from_raw_parts(ptr as *const u8, len as usize)
    }
}

#[no_mangle]
pub extern "C" fn initialize() {
    //
}

#[no_mangle]
pub extern "C" fn svm_alloc(size: i32) -> i32 {
    svm_sdk::memory::alloc(size as usize) as i32
}

#[no_mangle]
pub extern "C" fn store_addr() {
    let calldata = get_calldata();

    let mut cursor = Cursor::new(calldata);
    let decoder = Decoder::new();

    let addr: Address = decoder.decode_value(&mut cursor).unwrap().into();
    let ptr = addr.as_ptr() as i32;

    unsafe { store160(0, ptr, VAR_ID) };
}

#[no_mangle]
pub extern "C" fn load_addr() -> i32 {
    let ptr = svm_sdk::memory::alloc(20) as i32;

    unsafe { load160(VAR_ID, 0, ptr) };

    ptr
}
