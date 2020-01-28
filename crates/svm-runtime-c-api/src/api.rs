use std::{ffi::c_void, ptr::NonNull, string::FromUtf8Error};

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

macro_rules! addr_to_svm_byte_array {
    ($raw_byte_array:expr, $addr:expr) => {{
        to_svm_byte_array!($raw_byte_array, $addr.bytes(), Address::len());
    }};
}

macro_rules! state_to_svm_byte_array {
    ($raw_byte_array:expr, $state:expr) => {{
        to_svm_byte_array!($raw_byte_array, $state.bytes(), State::len());
    }};
}

macro_rules! vec_to_svm_byte_array {
    ($raw_byte_array:expr, $vec:expr) => {{
        to_svm_byte_array!($raw_byte_array, $vec, $vec.len());
    }};
}

macro_rules! to_svm_byte_array {
    ($raw_byte_array:expr, $src:expr, $length:expr) => {{
        use crate::svm_byte_array;
        use std::mem::ManuallyDrop;

        let byte_array: &mut svm_byte_array = &mut *$raw_byte_array;
        byte_array.bytes = $src.as_ptr();
        byte_array.length = $length as u32;

        ManuallyDrop::new($src);
    }};
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_imports_alloc(
    imports: *mut *mut Vec<svm_import_t>,
    length: u32,
) -> svm_result_t {
    let vec: Vec<svm_import_t> = Vec::with_capacity(length as usize);

    *imports = Box::into_raw(Box::new(vec));

    svm_result_t::SVM_SUCCESS
}

/// Builds a new `svm_import` (returned via `import` function parameter).
/// New built `svm_import_t` is pushed into `imports`
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_import_func_build(
    imports: *mut c_void,
    module_name: svm_byte_array,
    import_name: svm_byte_array,
    func: *const c_void,
    params: svm_value_type_array,
    returns: svm_value_type_array,
) -> svm_result_t {
    let imports = &mut *(imports as *mut Vec<svm_import_t>);

    assert!(imports.len() < imports.capacity());

    let func = NonNull::new(func as *mut c_void);
    if func.is_none() {
        todo!();
        return svm_result_t::SVM_FAILURE;
    }

    let func = svm_import_func_t {
        func: func.unwrap(),
        sig: svm_import_func_sig_t {
            params: params.into(),
            returns: returns.into(),
        },
    };

    let module_name: Result<String, FromUtf8Error> = module_name.into();
    let import_name: Result<String, FromUtf8Error> = import_name.into();

    if module_name.is_err() {
        todo!();
        return svm_result_t::SVM_FAILURE;
    }

    if import_name.is_err() {
        todo!();
        return svm_result_t::SVM_FAILURE;
    }

    let svm_import = svm_import_t {
        module_name: module_name.unwrap(),
        import_name: import_name.unwrap(),
        kind: svm_import_kind::SVM_FUNCTION,
        value: svm_import_value::Func(func),
    };

    imports.push(svm_import);

    svm_result_t::SVM_SUCCESS
}

/// Creates a new SVM Runtime instance.
/// Returns it via the `runtime` parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    runtime: *mut *mut c_void,
    path: svm_byte_array,
    host: *mut c_void,
    imports: *const c_void,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let path: Result<String, FromUtf8Error> = path.into();

    if let Err(_err) = path {
        todo!();
        // update_last_error(err);
        return svm_result_t::SVM_FAILURE;
    }

    let wasmer_imports = helpers::cast_imports_to_wasmer_imports(imports);

    let rt = svm_runtime::create_rocksdb_runtime::<String, DefaultJsonSerializerTypes>(
        host,
        &path.unwrap(),
        wasmer_imports,
    );

    let boxed_rt: Box<dyn Runtime> = Box::new(rt);
    let rt_ptr = RuntimePtr::new(boxed_rt);

    //  `svm_runtime_destroy` should be called later for freeing memory.
    *runtime = svm_common::into_raw_mut(rt_ptr);

    debug!("`svm_runtime_create` end");

    svm_result_t::SVM_SUCCESS
}

/// Deploys a new app-template
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_deploy_template(
    template_addr: *mut svm_byte_array,
    runtime: *mut c_void,
    author: *const c_void,
    host_ctx: svm_byte_array,
    template: svm_byte_array,
) -> svm_result_t {
    debug!("`svm_deploy_template` start`");

    let runtime = helpers::cast_to_runtime_mut(runtime);
    let author = Address::from(author);
    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);
    let bytes = std::slice::from_raw_parts(template.bytes, template.length as usize);

    if host_ctx.is_err() {
        todo!()
    }

    match runtime.deploy_template(&author, host_ctx.unwrap(), bytes) {
        Ok(addr) => {
            // returning deployed `AppTemplate` as `svm_byte_array`
            // client should call later `svm_address_destroy`
            addr_to_svm_byte_array!(template_addr, addr);

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
    app_addr: *mut svm_byte_array,
    init_state: *mut svm_byte_array,
    runtime: *mut c_void,
    creator: *const c_void,
    host_ctx: svm_byte_array,
    app: svm_byte_array,
) -> svm_result_t {
    debug!("`svm_spawn_app` start");

    let runtime = helpers::cast_to_runtime_mut(runtime);
    let creator = Address::from(creator);
    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);

    if host_ctx.is_err() {
        todo!();
        return svm_result_t::SVM_FAILURE;
    }

    let bytes = std::slice::from_raw_parts(app.bytes, app.length as usize);

    match runtime.spawn_app(&creator, host_ctx.unwrap(), bytes) {
        Ok((addr, state)) => {
            // returning spawned app `Address` as `svm_byte_array`
            // client should call later `svm_address_destroy`
            addr_to_svm_byte_array!(app_addr, addr);

            // returning spawned app initial `State` as `svm_byte_array`
            // client should call later `svm_state_destroy`
            state_to_svm_byte_array!(init_state, state);

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
    sender: *const c_void,
    tx: svm_byte_array,
) -> svm_result_t {
    debug!("`svm_parse_exec_app` start");

    let runtime = helpers::cast_to_runtime(runtime);
    let sender = Address::from(sender);
    let bytes = std::slice::from_raw_parts(tx.bytes, tx.length as usize);

    match runtime.parse_exec_app(&sender, bytes) {
        Ok(tx) => {
            // `AppTransaction` will be freed later as part `svm_exec_app`
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
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    app_tx: *const c_void,
    state: *const c_void,
    host_ctx: svm_byte_array,
) -> svm_result_t {
    debug!("`svm_exec_app` start");

    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);

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
            let vec = crate::receipt::encode_receipt(r);

            // returning encoded `Receipt` as `svm_byte_array`
            // should call later `svm_receipt_destroy`
            vec_to_svm_byte_array!(receipt, vec);

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

/// Returns a raw pointer to `the host` extracted from a raw pointer to `wasmer` context.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_instance_context_host_get(ctx: *mut c_void) -> *mut c_void {
    use wasmer_runtime_core::vm::Ctx;

    let wasmer_ctx = svm_common::from_raw::<Ctx>(ctx);
    let svm_ctx = svm_common::from_raw::<SvmCtx>(wasmer_ctx.data);

    svm_ctx.host
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_imports_destroy(imports: *mut c_void) -> svm_result_t {
    let _ = Box::from_raw(imports as *mut Vec<svm_import_t>);

    svm_result_t::SVM_SUCCESS
}

/// Destroys the Runtime and it's associated resources.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_destroy(runtime: *mut c_void) -> svm_result_t {
    debug!("`svm_runtime_destroy`");

    let _ = Box::from_raw(runtime as *mut RuntimePtr);

    svm_result_t::SVM_SUCCESS
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_address_destroy(address: *mut c_void) -> svm_result_t {
    let _ = Box::from_raw(address as *mut Address);

    svm_result_t::SVM_SUCCESS
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_state_destroy(state: *mut c_void) -> svm_result_t {
    let _ = Box::from_raw(state as *mut State);

    svm_result_t::SVM_SUCCESS
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_receipt_destroy(bytes: svm_byte_array) -> svm_result_t {
    todo!();

    svm_result_t::SVM_SUCCESS
}
