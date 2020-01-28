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
    imports: *const c_void,
) -> Vec<(String, String, Export)> {
    // function code has been influenced heavily by `wasmer_import_object_extend` here:
    // https://github.com/wasmerio/wasmer/blob/f9bb579c05abc795d597a03352683fc62a4121d5/lib/runtime-c-api/src/import/mod.rs#L373

    let mut res: Vec<(String, String, Export)> = Vec::new();

    let imports = &*(imports as *const Vec<svm_import_t>);

    for import in imports.iter() {
        let wasmer_import = match import.kind {
            svm_import_kind::SVM_FUNCTION => crate::wasmer::to_wasmer_import_func(import),
        };

        let module_name = import.module_name.clone();
        let import_name = import.import_name.clone();
        let import_tuple = (module_name, import_name, wasmer_import);

        res.push(import_tuple);
    }

    res
}
