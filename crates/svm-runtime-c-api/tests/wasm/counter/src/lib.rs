#![no_std]

use svm_abi_decoder::{Cursor, Decoder};

const VAR_ID: u32 = 0;

#[link(wasm_import_module = "svm")]
extern "C" {
    fn calldata_ptr() -> i32;

    fn calldata_len() -> i32;

    fn get32(var_id: u32) -> u32;

    fn set32(var_id: u32, value: u32);
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
    let calldata = get_calldata();

    let mut cursor = Cursor::new(calldata);
    let decoder = Decoder::new();

    let initial: u32 = decoder.decode_value(&mut cursor).unwrap().into();

    unsafe {
        set32(VAR_ID, initial);
    }
}

#[no_mangle]
pub extern "C" fn add() {
    let calldata = get_calldata();

    let mut cursor = Cursor::new(calldata);
    let decoder = Decoder::new();

    let addition: u32 = decoder.decode_value(&mut cursor).unwrap().into();

    unsafe {
        let old = get32(VAR_ID);
        let new = old + addition;

        set32(VAR_ID, new);
    }
}
