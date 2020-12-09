use crate::traits::Storage;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        fn svm_get32(var_id: u32) -> u32 {
            crate::MockStorage::get32(var_id)
        }

        fn svm_get64(var_id: u32) -> u64 {
            crate::MockStorage::get64(var_id)
        }

        fn svm_set32(var_id: u32, value: u32) {
            crate::MockStorage::set32(var_id, value);
        }

        fn svm_set64(var_id: u32, value: u64) {
            crate::MockStorage::set64(var_id, value);
        }

        fn svm_store160(offset: u32, var_id: u32) {
            crate::MockStorage::store160(offset, var_id as usize);
        }

        fn svm_load160(var_id: u32, offset: u32) {
            crate::MockStorage::load160(var_id, offset as usize);
        }
    }
    else {
        #[link(wasm_import_module = "svm")]
        extern "C" {
            fn svm_get32(var_id: u32) -> u32;

            fn svm_get64(var_id: u32) -> u64;

            fn svm_set32(var_id: u32, value: u32);

            fn svm_set64(var_id: u32, value: u64);

            fn svm_store160(offset: u32, var_id: u32);

            fn svm_load160(var_id: u32, offset: u32);
        }
    }
}

pub struct ExtStorage;

impl Storage for ExtStorage {
    fn get32(var_id: u32) -> u32 {
        unsafe { svm_get32(var_id) }
    }

    fn get64(var_id: u32) -> u64 {
        unsafe { svm_get64(var_id) }
    }

    fn set32(var_id: u32, value: u32) {
        unsafe { svm_set32(var_id, value) }
    }

    fn set64(var_id: u32, value: u64) {
        unsafe { svm_set64(var_id, value) }
    }

    fn store160(var_id: u32, offset: usize) {
        unsafe { svm_store160(offset as u32, var_id) }
    }

    fn load160(var_id: u32, offset: usize) {
        unsafe { svm_load160(var_id, offset as u32) }
    }
}
