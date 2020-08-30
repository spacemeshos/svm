use std::ffi::c_void;

use crate::svm_byte_array;

use svm_runtime::Context;

use wasmer_c_api::instance::wasmer_instance_context_t;

/// Given a raw pointer to `wasmer` instance context, mutably borrows inner `data`
/// and extract from it a pointer to the so called `host`.
/// (it's type is defined as `T` in thefunction declaration)
pub unsafe fn svm_host_get<'a, T>(raw_ctx: *mut wasmer_instance_context_t) -> &'a mut T {
    let ctx = cast_to_context(raw_ctx);
    let svm_ctx = svm_common::from_raw_mut::<SvmCtx>(ctx.data);

    &mut *(svm_ctx.host as *mut T)
}

/// Casts a raw pointer to wasmer instance context to it's Safe Rust version (`&mut Context`)
pub unsafe fn cast_to_context<'a>(ctx: *mut wasmer_instance_context_t) -> &'a mut Context {
    &mut *(ctx as *mut Context)
}

/// Allocates `count` imports array, returns a pointer to the first import.
pub fn imports_alloc(count: u32) -> *mut c_void {
    let mut imports = std::ptr::null_mut();

    let res = unsafe { crate::svm_imports_alloc(&mut imports, count) };
    assert!(res.is_ok());

    imports
}

/// Given an import function relevant data (module name, import name, function pointer, params and returns),
/// Allocates on the heap an `svm_import_t` instance holding raw pointers and other `svm_... raw types.
///
/// This allocated `svm_import_t` should be destroyed after not being required anymore.
/// see: `svm_import_func_destroy` under crate `api.rs`
pub unsafe fn import_func_create(
    imports: *mut c_void,
    module_name: &str,
    import_name: &str,
    func: *mut c_void,
    params: svm_byte_array,
    returns: svm_byte_array,
) {
    let mut error = svm_byte_array::default();

    let res = crate::svm_import_func_build(
        imports,
        module_name.into(),
        import_name.into(),
        func,
        params.into(),
        returns.into(),
        &mut error,
    );

    assert!(res.is_ok());
}
