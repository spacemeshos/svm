mod receipt;
pub use receipt::{decode_receipt, ClientReceipt};

use std::{cell::RefCell, ffi::c_void, rc::Rc};

use crate::{
    helpers, svm_byte_array, svm_result_t, svm_value_type, svm_value_type_array, RuntimePtr,
};
use log::debug;

use svm_kv::memory::MemKVStore;
use svm_runtime::{ctx::SvmCtx, traits::Runtime};

use wasmer_runtime_c_api::instance::wasmer_instance_context_t;
use wasmer_runtime_core::vm::Ctx;

/// Creates a new in-memory `MemKVStore`.
/// Returns a raw pointer to allocated kv-store via input parameter `raw_kv`
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_kv_create(kv: *mut *mut c_void) -> svm_result_t {
    let native_kv = svm_runtime::testing::memory_kv_store_init();
    *kv = svm_common::into_raw_mut(native_kv);

    svm_result_t::SVM_SUCCESS
}

/// Creates a new SVM in-memory Runtime instance.
/// Returns it via the `raw_runtime` parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(
    raw_runtime: *mut *mut c_void,
    kv: *const c_void,
    host: *mut c_void,
    imports: *const c_void,
) -> svm_result_t {
    debug!("`svm_memory_runtime_create` start");

    let kv: &Rc<RefCell<MemKVStore>> = &*(kv as *const Rc<RefCell<MemKVStore>>);
    let wasmer_imports = helpers::cast_imports_to_wasmer_imports(imports);
    let runtime = svm_runtime::testing::create_memory_runtime(host, kv, wasmer_imports);

    let runtime: Box<dyn Runtime> = Box::new(runtime);

    let runtime_ptr = RuntimePtr::new(runtime);
    *raw_runtime = svm_common::into_raw_mut(runtime_ptr);

    debug!("`svm_memory_runtime_create` end");

    svm_result_t::SVM_SUCCESS
}

/// Returns a raw pointer to `SVM` live instance register of type `reg_bits:reg_idx`
pub unsafe fn svm_register_get(
    raw_ctx: *mut wasmer_instance_context_t,
    reg_bits: u32,
    reg_idx: u32,
) -> *const u8 {
    let ctx = cast_to_wasmer_ctx(raw_ctx);
    let reg = svm_runtime::helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);

    reg.as_ptr()
}

/// Given a raw pointer to `wasmer` instance context, mutably borrows inner `data`
/// and extract from it a pointer to the so called `host`.
/// (it's type is defined as `T` in thefunction declaration)
pub unsafe fn svm_host_get<'a, T>(raw_ctx: *mut wasmer_instance_context_t) -> &'a mut T {
    let ctx = cast_to_wasmer_ctx(raw_ctx);
    let svm_ctx = svm_common::from_raw_mut::<SvmCtx>(ctx.data);

    &mut *(svm_ctx.host as *mut T)
}

/// Casts a raw pointer to wasmer instance context to it's Safe Rust version (`&mut Ctx`)
pub unsafe fn cast_to_wasmer_ctx<'a>(ctx: *mut wasmer_instance_context_t) -> &'a mut Ctx {
    &mut *(ctx as *mut Ctx)
}

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
    params: Vec<svm_value_type>,
    returns: Vec<svm_value_type>,
) {
    let res = crate::svm_import_func_build(
        imports,
        module_name.into(),
        import_name.into(),
        func,
        params.into(),
        returns.into(),
    );
    assert!(res.is_ok());
}
