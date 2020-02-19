mod receipt;
pub use receipt::{decode_receipt, ClientReceipt};

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

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
pub unsafe extern "C" fn svm_memory_kv_create(raw_kv: *mut *mut c_void) -> svm_result_t {
    let kv = svm_runtime::testing::memory_kv_store_init();
    *raw_kv = svm_common::into_raw_mut(kv);

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
    debug!("`svm_runtime_create` start");

    let kv: &Rc<RefCell<MemKVStore>> = &*(kv as *const Rc<RefCell<MemKVStore>>);
    let wasmer_imports = helpers::cast_imports_to_wasmer_imports(imports);
    let runtime = svm_runtime::testing::create_memory_runtime(host, kv, wasmer_imports);

    let runtime: Box<dyn Runtime> = Box::new(runtime);

    let runtime_ptr = RuntimePtr::new(runtime);
    *raw_runtime = svm_common::into_raw_mut(runtime_ptr);

    debug!("`svm_runtime_create` end");

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

/// Given a borrowed string, returns a raw pointer to its underlying bytes
/// wrapped within an `svm_byte_array` instance.
pub fn str_to_svm_byte_array(s: &str) -> svm_byte_array {
    let bytes = s.as_ptr();
    let length = s.len() as u32;

    svm_byte_array { bytes, length }
}

/// Givena a borrowed vector of `svm_value_type`, returns a raw pointer to its underlying data
/// wrapped within an `svm_value_type_array` instance.
pub fn svm_value_type_vec_to_array(vec: &Vec<svm_value_type>) -> svm_value_type_array {
    let length = vec.len() as u32;
    let types = vec.as_ptr();

    svm_value_type_array { types, length }
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
    let module_name = str_to_svm_byte_array(module_name);
    let import_name = str_to_svm_byte_array(import_name);

    let res = crate::svm_import_func_build(
        imports,
        module_name,
        import_name,
        func,
        svm_value_type_vec_to_array(&params),
        svm_value_type_vec_to_array(&returns),
    );
    assert_eq!(true, res.as_bool());
}
