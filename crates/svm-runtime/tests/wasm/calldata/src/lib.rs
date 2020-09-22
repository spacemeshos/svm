#![feature(vec_into_raw_parts)]
#![allow(improper_ctypes_definitions)]

use svm_abi_decoder::{Cursor, Decoder};
use svm_abi_encoder::Encoder;
use svm_sdk::value::Address;

const VAR_ID: u32 = 0;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[link(wasm_import_module = "svm")]
extern "C" {
    fn svm_calldata_offset() -> u32;

    fn svm_calldata_len() -> u32;

    fn svm_set_returndata(offset: u32, length: u32);

    fn svm_load160(var_id: u32, offset: u32);

    fn svm_store160(mem_idx: u32, var_id: u32);
}

fn get_calldata() -> &'static [u8] {
    unsafe {
        let ptr = svm_calldata_offset();
        let len = svm_calldata_len();

        core::slice::from_raw_parts(ptr as *const u8, len as usize)
    }
}

#[no_mangle]
pub extern "C" fn initialize() {
    //
}

#[no_mangle]
pub extern "C" fn svm_alloc(size: u32) -> u32 {
    svm_sdk::memory::alloc(size as usize) as u32
}

#[no_mangle]
pub extern "C" fn store_addr() {
    let calldata = get_calldata();

    let mut cursor = Cursor::new(calldata);
    let decoder = Decoder::new();

    let addr: Address = decoder.decode_value(&mut cursor).unwrap().into();
    let offset = addr.as_ptr() as usize as u32;

    unsafe { svm_store160(offset, VAR_ID) };
}

#[no_mangle]
pub extern "C" fn return_addr() {
    let addr = load_addr();

    let mut buf = Vec::new();
    addr.encode(&mut buf);

    let (ptr, len, _cap) = buf.into_raw_parts();

    unsafe { svm_set_returndata(ptr as u32, len as u32) }
}

fn load_addr() -> Address<'static> {
    let ptr = svm_sdk::memory::alloc(20) as u32;

    unsafe { svm_load160(VAR_ID, ptr) };

    let bytes: &[u8] = unsafe { core::slice::from_raw_parts(ptr as *const u8, 20) };

    bytes.into()
}
