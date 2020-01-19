use std::ffi::c_void;

use log::{debug, error};

use svm_app::{
    default::DefaultJsonSerializerTypes,
    types::{AppTransaction, HostCtx},
};
use svm_common::{Address, State};
use svm_runtime::{ctx::SvmCtx, traits::Runtime};

use crate::{
    helpers, svm_byte_array, svm_import_func_sig_t, svm_import_func_t, svm_import_kind,
    svm_import_t, svm_import_value, svm_result_t, svm_value_type_array, RuntimePtr,
};

/// Creates a new SVM Runtime instance.
/// Returns it via the `runtime` parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    runtime: *mut *mut c_void,
    path_bytes: *const c_void,
    path_len: u32,
    host: *mut c_void,
    imports: *const *const svm_import_t,
    imports_len: u32,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let slice = std::slice::from_raw_parts(path_bytes as *const u8, path_len as usize);
    let path = String::from_utf8(slice.to_vec());

    if let Err(_err) = path {
        // update_last_error(err);
        return svm_result_t::SVM_FAILURE;
    }

    let wasmer_imports = helpers::cast_imports_to_wasmer_imports(imports, imports_len);
    let rt = svm_runtime::create_rocksdb_runtime::<String, DefaultJsonSerializerTypes>(
        host,
        &path.unwrap(),
        wasmer_imports,
    );
    let boxed_rt: Box<dyn Runtime> = Box::new(rt);

    let rt_ptr = RuntimePtr::new(boxed_rt);
    *runtime = svm_common::into_raw_mut(rt_ptr);

    debug!("`svm_runtime_create` end");

    svm_result_t::SVM_SUCCESS
}

/// Destroys the Runtime and it's associated resources.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_destroy(runtime: *mut c_void) -> svm_result_t {
    debug!("`svm_runtime_destroy`");

    let _runtime: Box<RuntimePtr> = Box::from_raw(runtime as *mut RuntimePtr);

    svm_result_t::SVM_SUCCESS
}

/// Builds a new `svm_import` (returned via `import` function parameter).
/// Import `svm_import_t` is allocated on the heap.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_import_func_build(
    import: *mut *mut svm_import_t,
    module_name: svm_byte_array,
    import_name: svm_byte_array,
    func: *const c_void,
    params: svm_value_type_array,
    returns: svm_value_type_array,
) -> svm_result_t {
    let sig = svm_import_func_sig_t {
        params: params.types,
        returns: returns.types,
        params_len: params.types_len,
        returns_len: returns.types_len,
    };

    let func = svm_import_func_t { func, sig };
    let func = Box::into_raw(Box::new(func));

    let svm_import = svm_import_t {
        module_name,
        import_name,
        kind: svm_import_kind::SVM_FUNCTION,
        value: svm_import_value { func },
    };

    *import = Box::into_raw(Box::new(svm_import));

    svm_result_t::SVM_SUCCESS
}

/// Destroys `svm_import_t` resources.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_import_destroy(_func: *mut c_void) -> svm_result_t {
    todo!()
}

/// Deploys a new app-template
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_deploy_template(
    template_addr: *mut *mut c_void,
    runtime: *mut c_void,
    author_addr: *const c_void,
    host_ctx_bytes: *const c_void,
    host_ctx_len: u32,
    bytes: *const c_void,
    bytes_len: u32,
) -> svm_result_t {
    debug!("`svm_deploy_template` start`");

    let runtime = helpers::cast_to_runtime_mut(runtime);
    let author = Address::from(author_addr);
    let bytes = std::slice::from_raw_parts(bytes as *const u8, bytes_len as usize);
    let host_ctx = HostCtx::from_raw_parts(host_ctx_bytes, host_ctx_len as usize);

    if host_ctx.is_err() {
        todo!()
    }

    match runtime.deploy_template(&author, host_ctx.unwrap(), &bytes) {
        Ok(addr) => {
            *template_addr = svm_common::into_raw_mut(addr);
            debug!("`svm_deploy_template`` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(_err) => {
            // update_last_error(err);
            error!("`svm_deploy_template` returns `SVM_FAILURE`");
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Spawns a new App.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_spawn_app(
    app_addr: *mut *mut c_void,
    runtime: *mut c_void,
    creator_addr: *const c_void,
    host_ctx_bytes: *const c_void,
    host_ctx_len: u32,
    bytes: *const c_void,
    bytes_len: u32,
) -> svm_result_t {
    debug!("`svm_spawn_app` start");

    let runtime = helpers::cast_to_runtime_mut(runtime);
    let creator = Address::from(creator_addr);
    let host_ctx = HostCtx::from_raw_parts(host_ctx_bytes, host_ctx_len as usize);
    let bytes = std::slice::from_raw_parts(bytes as *const u8, bytes_len as usize);

    match runtime.spawn_app(&creator, host_ctx.unwrap(), bytes) {
        Ok(addr) => {
            *app_addr = svm_common::into_raw_mut(addr);
            debug!("`svm_spawn_app` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(_e) => {
            // update_last_error(error);
            error!("`svm_spawn_app` returns `SVM_FAILURE`");
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Parses `exec-app` raw transaction into an `AppTransaction`.
/// Returns a raw reference via `app_tx` function parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_parse_exec_app(
    app_tx: *mut *mut c_void,
    runtime: *const c_void,
    sender_addr: *const c_void,
    bytes: *const c_void,
    bytes_len: u32,
) -> svm_result_t {
    debug!("`svm_parse_exec_app` start");

    let runtime = helpers::cast_to_runtime(runtime);
    let sender = Address::from(sender_addr);
    let bytes = std::slice::from_raw_parts(bytes as *const u8, bytes_len as usize);

    match runtime.parse_exec_app(&sender, bytes) {
        Ok(tx) => {
            *app_tx = svm_common::into_raw_mut(tx);
            debug!("`svm_parse_exec_app` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(_error) => {
            // update_last_error(error);
            error!("`svm_parse_exec_app` returns `SVM_FAILURE`");
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Triggers an app-transaction execution of an already deployed app.
///
/// Returns the receipt of the execution via the `receipt` parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_exec_app(
    receipt: *mut *mut c_void,
    receipt_length: *mut u32,
    runtime: *mut c_void,
    app_tx: *const c_void,
    state: *const c_void,
    host_ctx_bytes: *const c_void,
    host_ctx_len: u32,
) -> svm_result_t {
    debug!("`svm_exec_app` start");

    let host_ctx = HostCtx::from_raw_parts(host_ctx_bytes, host_ctx_len as usize);

    if host_ctx.is_err() {
        // update_last_error(e);
        error!("`svm_exec_app` returns `SVM_FAILURE`");
        return svm_result_t::SVM_FAILURE;
    }

    let host_ctx = host_ctx.unwrap();
    let app_tx = *Box::from_raw(app_tx as *mut AppTransaction);
    let runtime = helpers::cast_to_runtime_mut(runtime);
    let state = State::from(state);

    match runtime.exec_app(app_tx, state, host_ctx) {
        Ok(ref r) => {
            let mut bytes = crate::receipt::encode_receipt(r);

            *receipt_length = bytes.len() as u32;
            *receipt = bytes.as_mut_ptr() as _;
            std::mem::forget(bytes);

            debug!("`svm_exec_app` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(_e) => {
            // update_last_error(e);
            error!("`svm_exec_app` returns `SVM_FAILURE`");
            svm_result_t::SVM_FAILURE
        }
    }
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_receipt_destroy(_receipt: *mut c_void) -> svm_result_t {
    todo!()
}

/// Returns a raw pointer to `the host` extracted from a raw pointer to `wasmer` context.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_instance_context_host_get(ctx: *mut c_void) -> *mut c_void {
    use wasmer_runtime_core::vm::Ctx;

    let wasmer_ctx = svm_common::from_raw::<Ctx>(ctx);
    let svm_ctx = svm_common::from_raw::<SvmCtx>(wasmer_ctx.data);

    svm_ctx.host
}
