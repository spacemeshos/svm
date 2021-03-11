#![feature(vec_into_raw_parts)]
#![allow(improper_ctypes_definitions)]

extern crate svm_sdk;

use svm_sdk::traits::Encoder;
use svm_sdk::{Address, CallData};

const VAR_ID: u32 = 0;

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
    let ptr = svm_sdk::alloc(size as usize);

    ptr.offset() as u32
}

#[no_mangle]
pub extern "C" fn store_addr() {
    let bytes = get_calldata();
    let mut calldata = CallData::new(bytes);

    let addr: Address = calldata.next_1();
    let offset = addr.offset() as u32;

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

fn load_addr() -> Address {
    let ptr = svm_sdk::alloc(Address::len());
    let off = ptr.offset() as u32;

    unsafe { svm_load160(VAR_ID, off) };

    let bytes: &[u8] = unsafe { core::slice::from_raw_parts(ptr.as_ptr(), Address::len()) };

    bytes.into()
}
