use wasmer_runtime_core::export::Export;

#[repr(C)]
pub struct svm_byte_array {
    pub bytes: *const u8,
    pub bytes_len: u32,
}

#[repr(C)]
pub enum svm_import_kind {
    FUNCTION = 0,
}

#[repr(C)]
pub struct svm_import_value {
    //
}

#[repr(C)]
pub struct svm_import_t {
    pub module_bytes: svm_byte_array,
    pub import_name: svm_byte_array,
    pub kind: svm_import_kind,
    pub value: svm_import_value,
}

pub(crate) fn to_wasmer_import(import: *mut svm_import_t) -> (String, String, Export) {
    todo!()
}
