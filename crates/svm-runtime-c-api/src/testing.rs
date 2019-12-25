use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::Arc;

use crate::{helpers, RuntimePtr};
use log::debug;

use svm_kv::memory::MemKVStore;

use wasmer_runtime_c_api::{
    export::{wasmer_import_export_kind, wasmer_import_export_value},
    import::{wasmer_import_func_new, wasmer_import_func_t, wasmer_import_t},
    instance::wasmer_instance_context_t,
    value::wasmer_value_tag,
    wasmer_byte_array, wasmer_result_t,
};

use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
    vm::{Ctx, Func},
};

use svm_runtime::{ctx::SvmCtx, helpers::cast_ptr_to_svm_ctx, traits::Runtime};

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
) -> wasmer_result_t {
    debug!("`svm_runtime_create` start");

    let kv: &Rc<RefCell<MemKVStore>> = &*(kv as *const Rc<RefCell<MemKVStore>>);
    let imports = helpers::cast_host_imports(imports, imports_len);
    let runtime = svm_runtime::testing::create_memory_runtime(host, kv, imports);

    let runtime: Box<dyn Runtime> = Box::new(runtime);

    let runtime_ptr = RuntimePtr::new(runtime);
    *raw_runtime = helpers::into_raw_mut(runtime_ptr);

    debug!("`svm_runtime_create` end");

    wasmer_result_t::WASMER_OK
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
    let svm_ctx = cast_ptr_to_svm_ctx(ctx.data);

    &mut *(svm_ctx.host as *mut T)
}

pub unsafe fn cast_to_wasmer_ctx<'a>(ctx: *mut wasmer_instance_context_t) -> &'a mut Ctx {
    &mut *(ctx as *mut Ctx)
}

pub unsafe fn alloc_ptr() -> *mut c_void {
    let ptr: *mut c_void = std::ptr::null_mut();
    let ptr = Box::new(ptr);

    *Box::into_raw(ptr)
}

pub unsafe fn wasmer_import_func_build(
    func: *mut c_void,
    params: Vec<Type>,
    returns: Vec<Type>,
) -> *mut wasmer_import_func_t {
    let func: *const Func = &Func(func) as _;

    let export = Export::Function {
        func: FuncPointer::new(func),
        ctx: Context::Internal,
        signature: Arc::new(FuncSig::new(params, returns)),
    };

    Box::into_raw(Box::new(export)) as _
}

pub unsafe fn wasmer_import_func_destroy(func: *mut wasmer_import_func_t) {
    Box::from_raw(func);
}

pub fn str_to_bytes(s: &str) -> (*const u8, u32) {
    let bytes = s.as_ptr();
    let bytes_len = s.len() as u32;

    (bytes, bytes_len)
}

pub unsafe fn wasmer_import_func_create(
    module_name: &str,
    import_name: &str,
    func: *mut c_void,
    params: Vec<Type>,
    returns: Vec<Type>,
) -> wasmer_import_t {
    let module_name = str_to_wasmer_byte_array(module_name);
    let import_name = str_to_wasmer_byte_array(import_name);
    let func = wasmer_import_func_build(func, params, returns);

    wasmer_import_t {
        module_name,
        import_name,
        tag: wasmer_import_export_kind::WASM_FUNCTION,
        value: wasmer_import_as_value(func),
    }
}

fn str_to_wasmer_byte_array(s: &str) -> wasmer_byte_array {
    let (bytes, bytes_len) = str_to_bytes(s);

    wasmer_byte_array { bytes, bytes_len }
}

fn wasmer_import_as_value(func: *const wasmer_import_func_t) -> wasmer_import_export_value {
    wasmer_import_export_value { func }
}
