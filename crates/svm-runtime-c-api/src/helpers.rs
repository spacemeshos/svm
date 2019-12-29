use std::ffi::c_void;
use std::slice;

use crate::{
    svm_import_func_sig_t, svm_import_func_t, svm_import_kind, svm_import_t, svm_value_type,
    RuntimePtr,
};
use svm_runtime::traits::Runtime;

use wasmer_runtime_core::export::Export;

#[inline(always)]
pub unsafe fn cast_to_runtime<'a>(raw_runtime: *const c_void) -> &'a Box<dyn Runtime> {
    &*(raw_runtime as *const RuntimePtr)
}

#[inline(always)]
pub unsafe fn cast_to_runtime_mut<'a>(raw_runtime: *mut c_void) -> &'a mut Box<dyn Runtime> {
    &mut *(raw_runtime as *mut RuntimePtr)
}

pub unsafe fn cast_host_imports(
    imports: *mut c_void,
    imports_len: libc::c_uint,
) -> Vec<(String, String, Export)> {
    // function code has been influenced heavily by `wasmer_import_object_extend` here:
    // https://github.com/wasmerio/wasmer/blob/f9bb579c05abc795d597a03352683fc62a4121d5/lib/runtime-c-api/src/import/mod.rs#L373

    let mut res: Vec<(String, String, Export)> = Vec::new();

    let imports: &[svm_import_t] = slice::from_raw_parts(imports as _, imports_len as usize);

    for import in imports {
        let module_name = slice::from_raw_parts(
            import.module_name.bytes,
            import.module_name.bytes_len as usize,
        );
        let module_name = if let Ok(s) = std::str::from_utf8(module_name) {
            s
        } else {
            panic!("error converting `module_name` to string".to_string());
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

        let wasmer_import = match import.kind {
            svm_import_kind::SVM_FUNCTION => {
                crate::wasmer::to_wasmer_import_func(import.value.func)
            }
            _ => todo!(),
        };

        let import_tuple = (
            module_name.to_string(),
            import_name.to_string(),
            wasmer_import,
        );
        res.push(import_tuple);
    }

    res
}
