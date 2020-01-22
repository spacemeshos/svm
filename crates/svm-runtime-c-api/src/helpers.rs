use std::ffi::c_void;
use std::slice;

use crate::{svm_import_kind, svm_import_t, RuntimePtr};
use svm_runtime::traits::Runtime;

use wasmer_runtime_core::export::Export;

/// Casts raw pointer to borrowed Runtime
#[inline]
pub unsafe fn cast_to_runtime<'a>(raw_runtime: *const c_void) -> &'a Box<dyn Runtime> {
    &*(raw_runtime as *const RuntimePtr)
}

/// Casts raw pointer to mutably borrowed Runtime
#[inline]
pub unsafe fn cast_to_runtime_mut<'a>(raw_runtime: *mut c_void) -> &'a mut Box<dyn Runtime> {
    &mut *(raw_runtime as *mut RuntimePtr)
}

/// Casts a raw array of `svm_import_t` into `wasmer` a vector of `(String, String, Export)`.
///
/// * The first tuple `String` signifes the import module name.
/// * The second tuple `String` signifes the import function name.
/// * The last `Export` is `wasmer` inner representation for imports.
pub unsafe fn cast_imports_to_wasmer_imports(
    imports: *const *const svm_import_t,
    imports_len: u32,
) -> Vec<(String, String, Export)> {
    // function code has been influenced heavily by `wasmer_import_object_extend` here:
    // https://github.com/wasmerio/wasmer/blob/f9bb579c05abc795d597a03352683fc62a4121d5/lib/runtime-c-api/src/import/mod.rs#L373

    let mut res: Vec<(String, String, Export)> = Vec::new();

    let imports: &[*const svm_import_t] = slice::from_raw_parts(imports as _, imports_len as usize);

    for import in imports {
        let import: &svm_import_t = &**import;

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
