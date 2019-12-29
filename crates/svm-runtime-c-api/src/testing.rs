use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::Arc;

use crate::{
    helpers, svm_byte_array, svm_import_func_sig_t, svm_import_func_t, svm_import_kind,
    svm_import_t, svm_import_value, svm_result_t, svm_value_type, RuntimePtr,
};
use log::debug;

use svm_kv::memory::MemKVStore;
use svm_runtime::{ctx::SvmCtx, traits::Runtime};

use wasmer_runtime_c_api::instance::wasmer_instance_context_t;
use wasmer_runtime_core::vm::Ctx;

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_kv_create(raw_kv: *mut *mut c_void) {
    let kv = svm_runtime::testing::memory_kv_store_init();
    *raw_kv = svm_common::into_raw_mut(kv);
}

/// Creates a new SVM in-memory Runtime instance.
/// Returns it via the `raw_runtime` parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(
    raw_runtime: *mut *mut c_void,
    kv: *const c_void,
    host: *mut c_void,
    imports: *mut c_void,
    imports_len: libc::c_uint,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let kv: &Rc<RefCell<MemKVStore>> = &*(kv as *const Rc<RefCell<MemKVStore>>);
    let imports = helpers::cast_host_imports(imports, imports_len);
    let runtime = svm_runtime::testing::create_memory_runtime(host, kv, imports);

    let runtime: Box<dyn Runtime> = Box::new(runtime);

    let runtime_ptr = RuntimePtr::new(runtime);
    *raw_runtime = svm_common::into_raw_mut(runtime_ptr);

    debug!("`svm_runtime_create` end");

    svm_result_t::SVM_SUCCESS
}

pub unsafe fn svm_register_get(
    raw_ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
) -> *const u8 {
    let ctx = cast_to_wasmer_ctx(raw_ctx);
    let reg = svm_runtime::helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.as_ptr()
}

pub unsafe fn svm_host_get<'a, T>(raw_ctx: *mut wasmer_instance_context_t) -> &'a mut T {
    let ctx = cast_to_wasmer_ctx(raw_ctx);
    let svm_ctx = svm_common::from_raw_mut::<SvmCtx>(ctx.data);

    &mut *(svm_ctx.host as *mut T)
}

pub unsafe fn cast_to_wasmer_ctx<'a>(ctx: *mut wasmer_instance_context_t) -> &'a mut Ctx {
    &mut *(ctx as *mut Ctx)
}

pub unsafe fn svm_import_func_destroy(func: *mut svm_import_func_t) {
    Box::from_raw(func);
}

pub fn str_to_bytes(s: &str) -> (*const u8, u32) {
    let bytes = s.as_ptr();
    let bytes_len = s.len() as u32;

    (bytes, bytes_len)
}

pub unsafe fn import_func_create(
    module_name: &str,
    import_name: &str,
    func: *const c_void,
    params: Vec<svm_value_type>,
    returns: Vec<svm_value_type>,
) -> svm_import_t {
    let module_name = str_to_svm_byte_array(module_name);
    let import_name = str_to_svm_byte_array(import_name);
    let func = svm_import_func_build(func, params, returns);
    let func: *mut svm_import_func_t = Box::into_raw(Box::new(func));

    svm_import_t {
        module_name,
        import_name,
        kind: svm_import_kind::SVM_FUNCTION,
        value: svm_import_value {
            func: func as *mut c_void,
        },
    }
}

fn str_to_svm_byte_array(s: &str) -> svm_byte_array {
    let (bytes, bytes_len) = str_to_bytes(s);

    svm_byte_array { bytes, bytes_len }
}

fn svm_import_func_build(
    func: *const c_void,
    params: Vec<svm_value_type>,
    returns: Vec<svm_value_type>,
) -> svm_import_func_t {
    let params_len = params.len() as u32;
    let returns_len = returns.len() as u32;

    let sig = svm_import_func_sig_t {
        params: params.as_ptr(),
        params_len,
        returns: returns.as_ptr(),
        returns_len,
    };

    svm_import_func_t { func, sig }
}
