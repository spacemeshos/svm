extern crate svm_sdk;

#[link(wasm_import_module = "host")]
extern "C" {
    fn host_fail();
}

#[no_mangle]
pub extern "C" fn svm_alloc(size: i32) -> i32 {
    let ptr = svm_sdk::alloc(size as usize);

    ptr.offset() as i32
}

#[no_mangle]
pub extern "C" fn initialize() {
    //
}

#[no_mangle]
pub extern "C" fn fail() {
    unsafe { host_fail() }
}
