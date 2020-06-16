mod spawn_app;

#[no_mangle]
pub unsafe extern "C" fn allocate(length: i32) -> i32 {
    let buf = Vec::with_capacity(length as usize);

    let (ptr, len, cap) = buf.into_raw_parts();

    ptr as *const u8 as i32
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: i32, length: i32) {
    let ptr = ptr as *mut u8;
    let len = length as usize;
    let cap = len;

    let _vec = Vec::from_raw_parts(ptr, len, cap);
}
