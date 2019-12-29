use wasmer_runtime_core::export::Export;

#[repr(C)]
pub struct svm_byte_array {
    pub bytes: *const u8,
    pub bytes_len: u32,
}

#[repr(u32)]
pub enum svm_import_kind {
    SVM_FUNCTION = 0,
}

#[repr(C)]
pub union svm_import_value {
    pub func: *const svm_import_func_t,
}

struct svm_import_func_t;

#[repr(C)]
pub struct svm_import_t {
    pub module_name: svm_byte_array,
    pub import_name: svm_byte_array,
    pub kind: svm_import_kind,
    pub value: svm_import_value,
}

pub(crate) fn to_wasmer_import(import: *mut svm_import_t) -> (String, String, Export) {
    let module_name = slice::from_raw_parts(
        import.module_name.bytes,
        import.module_name.bytes_len as usize,
    );
    let module_name = if let Ok(s) = std::str::from_utf8(module_name) {
        s
    } else {
        panic!("error converting module name to string".to_string());
    };

    let import_name = slice::from_raw_parts(
        import.import_name.bytes,
        import.import_name.bytes_len as usize,
    );

    let import_name = if let Ok(s) = std::str::from_utf8(import_name) {
        s
    } else {
        panic!("error converting import_name to string".to_string());
    };
}
