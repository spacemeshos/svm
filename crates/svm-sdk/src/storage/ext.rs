include!("../externs.rs");

pub struct Storage;

impl Storage {
    pub fn get32(var_id: u32) -> u32 {
        unsafe { svm_get32(var_id) }
    }

    pub fn get64(var_id: u32) -> u64 {
        unsafe { svm_get64(var_id) }
    }

    pub fn set32(var_id: u32, value: u32) {
        unsafe { svm_set32(var_id, value) }
    }

    pub fn set64(var_id: u32, value: u64) {
        unsafe { svm_set64(var_id, value) }
    }

    pub fn store160(var_id: u32, ptr: usize) {
        unsafe { svm_store160(ptr as u32, var_id) }
    }

    pub fn load160(var_id: u32, ptr: usize) {
        unsafe { svm_load160(var_id, ptr as u32) }
    }
}
