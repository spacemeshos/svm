use wasmer_runtime::ImportObject;

/// A marker for the C representation of `ImportObject`
#[repr(C)]
pub struct wasmer_import_object_t;

/// Frees memory of the given ImportObject
#[no_mangle]
pub extern "C" fn wasmer_import_object_destroy(import_object: *mut wasmer_import_object_t) {
    if !import_object.is_null() {
        unsafe { Box::from_raw(import_object as *mut ImportObject) };
    }
}
