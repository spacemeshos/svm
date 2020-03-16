use std::{convert::TryFrom, ffi::c_void, ptr::NonNull, string::FromUtf8Error};

use log::{debug, error};

use svm_app::{
    default::DefaultSerializerTypes,
    types::{AppTransaction, HostCtx},
};
use svm_common::{Address, State};
use svm_runtime::{ctx::SvmCtx, gas::DefaultGasEstimator, Runtime};

use crate::{
    helpers, svm_byte_array, svm_import_func_sig_t, svm_import_func_t, svm_import_kind,
    svm_import_t, svm_import_value, svm_result_t, svm_value_type_array, RuntimePtr,
};

macro_rules! addr_to_svm_byte_array {
    ($raw_byte_array:expr, $addr:expr) => {{
        let (ptr, _len, _cap) = $addr.into_raw_parts();
        to_svm_byte_array!($raw_byte_array, ptr, Address::len());
    }};
}

macro_rules! state_to_svm_byte_array {
    ($raw_byte_array:expr, $state:expr) => {{
        let (ptr, _len, _cap) = $state.into_raw_parts();
        to_svm_byte_array!($raw_byte_array, ptr, State::len());
    }};
}

macro_rules! vec_to_svm_byte_array {
    ($raw_byte_array:expr, $vec:expr) => {{
        let len = $vec.len();
        $vec.truncate(len);

        let (ptr, _len, _cap) = $vec.into_raw_parts();

        to_svm_byte_array!($raw_byte_array, ptr, len);
    }};
}

macro_rules! to_svm_byte_array {
    ($raw_byte_array:expr, $ptr:expr, $length:expr) => {{
        use crate::svm_byte_array;

        let bytes: &mut svm_byte_array = &mut *$raw_byte_array;
        bytes.bytes = $ptr;
        bytes.length = $length as u32;
    }};
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_template(
    runtime: *const c_void,
    bytes: svm_byte_array,
) -> svm_result_t {
    let runtime = helpers::cast_to_runtime(runtime);
    let bytes = std::slice::from_raw_parts(bytes.bytes, bytes.length as usize);

    match runtime.validate_template(bytes) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => svm_result_t::SVM_FAILURE,
    }
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_app(
    runtime: *const c_void,
    bytes: svm_byte_array,
) -> svm_result_t {
    let runtime = helpers::cast_to_runtime(runtime);
    let bytes = std::slice::from_raw_parts(bytes.bytes, bytes.length as usize);

    match runtime.validate_app(bytes) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => svm_result_t::SVM_FAILURE,
    }
}

/// Parses `exec-app` raw transaction.
/// Returns the `App` address that appears in the transaction.
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
/// use svm_common::Address;
///
/// let mut host = std::ptr::null_mut();
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports) };
/// assert!(res.is_ok());
///
/// let mut app_tx = std::ptr::null_mut();
/// let tx = vec![0x00, 0x01, 0x2, 0x3].into();
/// let _res = unsafe { svm_parse_exec_app(&mut app_tx, runtime, tx) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_tx(
    app_tx: *mut svm_byte_array,
    runtime: *const c_void,
    bytes: svm_byte_array,
) -> svm_result_t {
    debug!("`svm_validate_tx` start");

    let runtime = helpers::cast_to_runtime(runtime);
    let bytes = std::slice::from_raw_parts(bytes.bytes, bytes.length as usize);

    todo!()

    // match runtime.validate_tx(bytes) {
    //     Ok(tx) => {
    //         // `AppTransaction` will be freed later as part `svm_exec_app`
    //         *app_tx = svm_common::into_raw_mut(tx);

    //         debug!("`svm_validate_tx` returns `SVM_SUCCESS`");
    //         svm_result_t::SVM_SUCCESS
    //     }
    //     Err(_e) => {
    //         // update_last_error(error);
    //         error!("`svm_validate_tx` returns `SVM_FAILURE`");
    //         svm_result_t::SVM_FAILURE
    //     }
    // }
}

/// Allocates space for the host imports.
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::svm_imports_alloc;
///
/// let count = 2;
/// let mut imports = std::ptr::null_mut();
///
/// let res = unsafe { svm_imports_alloc(&mut imports, count) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_imports_alloc(imports: *mut *mut c_void, count: u32) -> svm_result_t {
    let vec: Vec<svm_import_t> = Vec::with_capacity(count as usize);

    *imports = svm_common::into_raw_mut(vec);

    svm_result_t::SVM_SUCCESS
}

/// Builds a new `svm_import` (returned via `import` function parameter).
/// New built `svm_import_t` is pushed into `imports`
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// fn foo() {
///   // ...
/// }
///
/// // allocate one imports
/// let mut imports = testing::imports_alloc(1);
///
/// let module_name = "env".into();
/// let import_name = "foo".into();
/// let params = vec![].into();
/// let returns = vec![].into();
/// let func = foo as *const std::ffi::c_void;
///
/// let res = unsafe { svm_import_func_build(imports, module_name, import_name, func, params, returns) };
/// assert!(res.is_ok());
/// ```
///
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
        // return svm_result_t::SVM_FAILURE;
    }

    let func = svm_import_func_t {
        func: func.unwrap(),
        sig: svm_import_func_sig_t {
            params: params.into(),
            returns: returns.into(),
        },
    };

    let module_name = String::try_from(module_name);
    let import_name = String::try_from(import_name);

    if module_name.is_err() {
        todo!();
        // return svm_result_t::SVM_FAILURE;
    }

    if import_name.is_err() {
        todo!();
        // return svm_result_t::SVM_FAILURE;
    }

    let import = svm_import_t {
        module_name: module_name.unwrap(),
        import_name: import_name.unwrap(),
        kind: svm_import_kind::SVM_FUNCTION,
        value: svm_import_value::Func(func),
    };

    imports.push(import);

    svm_result_t::SVM_SUCCESS
}

macro_rules! box_runtime {
    ($raw_runtime:expr, $runtime:expr) => {{
        let runtime_ptr = RuntimePtr::new(Box::new($runtime));

        //  `svm_runtime_destroy` should be called later for freeing memory.
        *$raw_runtime = svm_common::into_raw_mut(runtime_ptr);

        svm_result_t::SVM_SUCCESS
    }};
}

/// Creates a new in-memory `MemKVStore`.
/// Returns a raw pointer to allocated kv-store via input parameter `raw_kv`.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_kv_create(kv: *mut *mut c_void) -> svm_result_t {
    let native_kv = svm_runtime::testing::memory_kv_store_init();
    *kv = svm_common::into_raw_mut(native_kv);

    svm_result_t::SVM_SUCCESS
}

/// Creates a new SVM Runtime instance baced-by an in-memory KV.
/// Returns it via the `runtime` parameter.
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// let mut runtime = std::ptr::null_mut();
/// let host = std::ptr::null_mut();
/// let mut imports = testing::imports_alloc(0);
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(
    runtime: *mut *mut c_void,
    kv: *mut c_void,
    host: *mut c_void,
    imports: *const c_void,
) -> svm_result_t {
    debug!("`svm_memory_runtime_create` start");

    let imports = helpers::cast_imports_to_wasmer_imports(imports);

    let kv = svm_common::from_raw_mut(kv);
    let memory_runtime = svm_runtime::testing::create_memory_runtime(host, kv, imports);

    let res = box_runtime!(runtime, memory_runtime);

    debug!("`svm_memory_runtime_create` end");

    res
}

/// Creates a new SVM Runtime instance.
/// Returns it via the `runtime` parameter.
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
///
/// let mut runtime = std::ptr::null_mut();
/// let path = "path goes here".into();
/// let host = std::ptr::null_mut();
/// let mut imports = testing::imports_alloc(0);
///
/// let res = unsafe { svm_runtime_create(&mut runtime, path, host, imports) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    runtime: *mut *mut c_void,
    path: svm_byte_array,
    host: *mut c_void,
    imports: *const c_void,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let path = String::try_from(path);

    if let Err(_err) = path {
        todo!();
        // update_last_error(err);
        // return svm_result_t::SVM_FAILURE;
    }

    let imports = helpers::cast_imports_to_wasmer_imports(imports);

    let rocksdb_runtime = svm_runtime::create_rocksdb_runtime::<
        String,
        DefaultSerializerTypes,
        DefaultGasEstimator,
    >(host, &path.unwrap(), imports);

    let res = box_runtime!(runtime, rocksdb_runtime);

    debug!("`svm_runtime_create` end");

    res
}

/// Deploys a new app-template
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
/// use svm_common::Address;
///
/// let mut host = std::ptr::null_mut();
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports) };
/// assert!(res.is_ok());
///
/// // deploy template
/// let mut template_addr = svm_byte_array::default();
/// let author: svm_byte_array = Address::of("@author").into();
/// let host_ctx = svm_byte_array::default();
/// let template = vec![0x0C, 0x00, 0x0D, 0x0E].into();
/// let res = unsafe { svm_deploy_template(&mut template_addr, runtime, author, host_ctx, template) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_deploy_template(
    template_addr: *mut svm_byte_array,
    runtime: *mut c_void,
    author: svm_byte_array,
    host_ctx: svm_byte_array,
    template: svm_byte_array,
) -> svm_result_t {
    debug!("`svm_deploy_template` start`");

    let runtime = helpers::cast_to_runtime_mut(runtime);
    let author: Result<Address, String> = author.into();

    if let Err(msg) = author {
        todo!()
        // return svm_result_t::SVM_FAILURE;
    }

    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);
    if host_ctx.is_err() {
        todo!();
        // return svm_result_t::SVM_FAILURE;
    }

    let bytes = std::slice::from_raw_parts(template.bytes, template.length as usize);

    let receipt = runtime.deploy_template(bytes, &author.unwrap().into(), host_ctx.unwrap(), false);
    todo!()

    //     Ok(addr) => {
    //         // returning deployed `AppTemplate` as `svm_byte_array`
    //         // client should call later `svm_address_destroy`
    //         addr_to_svm_byte_array!(template_addr, addr.unwrap());

    //         debug!("`svm_deploy_template`` returns `SVM_SUCCESS`");

    //         svm_result_t::SVM_SUCCESS
    //     }
    //     Err(_err) => {
    //         // update_last_error(err);
    //         error!("`svm_deploy_template` returns `SVM_FAILURE`");
    //         svm_result_t::SVM_FAILURE
    //     }
    // }
}

/// Spawns a new App.
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
/// use svm_common::Address;
///
/// let mut host = std::ptr::null_mut();
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports) };
/// assert!(res.is_ok());
///
/// let mut app_addr = svm_byte_array::default();
/// let mut init_state = svm_byte_array::default();
/// let creator = Address::of("@creator").into();
/// let mut init_state = svm_byte_array::default();
/// let host_ctx = svm_byte_array::default();
/// let app = svm_byte_array::default();
///
/// let _res = unsafe { svm_spawn_app(&mut app_addr, &mut init_state, runtime, creator, host_ctx, app) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_spawn_app(
    app_addr: *mut svm_byte_array,
    init_state: *mut svm_byte_array,
    runtime: *mut c_void,
    bytes: svm_byte_array,
    creator: svm_byte_array,
    host_ctx: svm_byte_array,
) -> svm_result_t {
    debug!("`svm_spawn_app` start");

    let runtime = helpers::cast_to_runtime_mut(runtime);
    let creator: Result<Address, String> = creator.into();

    if let Err(msg) = creator {
        todo!();
        // return svm_result_t::SVM_FAILURE;
    }

    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);
    if host_ctx.is_err() {
        todo!();
        // return svm_result_t::SVM_FAILURE;
    }

    let bytes = std::slice::from_raw_parts(bytes.bytes, bytes.length as usize);

    let receipt = runtime.spawn_app(bytes, &creator.unwrap().into(), host_ctx.unwrap(), false);
    todo!()

    //     Ok((addr, state)) => {
    //         // returning spawned app `Address` as `svm_byte_array`
    //         // client should call later `svm_address_destroy`
    //         addr_to_svm_byte_array!(app_addr, addr.unwrap());

    //         // returning spawned app initial `State` as `svm_byte_array`
    //         // client should call later `svm_state_destroy`
    //         state_to_svm_byte_array!(init_state, state);

    //         debug!("`svm_spawn_app` returns `SVM_SUCCESS`");
    //         svm_result_t::SVM_SUCCESS
    //     }
    //     Err(_e) => {
    //         // update_last_error(error);
    //         error!("`svm_spawn_app` returns `SVM_FAILURE`");
    //         svm_result_t::SVM_FAILURE
    //     }
    // }
}

/// Triggers an app-transaction execution of an already deployed app.
/// Returns the receipt of the execution via the `receipt` parameter.
///
/// # Example
///
/// ```rust, no_run
/// use std::ffi::c_void;
///
/// use svm_runtime_c_api::*;
/// use svm_app::types::AppTransaction;
/// use svm_common::{State, Address};
///
/// let mut host = std::ptr::null_mut();
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports) };
/// assert!(res.is_ok());
///
/// // `app_tx` should be parsed from bytes using `svm_parse_exec_app`
/// let app = Address::of("@app").into();
/// let app_tx = AppTransaction {
///     version: 0,
///     app,
///     func_idx: 0,
///     func_buf: Vec::new(),
///     func_args: Vec::new()
/// };
///
/// let app_tx_ptr = &app_tx as *const AppTransaction as *const c_void;
/// let state = State::empty().into();
/// let mut receipt = svm_byte_array::default();
/// let host_ctx = svm_byte_array::default();
/// let dry_run = false;
/// let _res = unsafe { svm_exec_app(&mut receipt, runtime, app_tx_ptr, state, host_ctx, dry_run) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_exec_app(
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    bytes: svm_byte_array,
    state: svm_byte_array,
    host_ctx: svm_byte_array,
    dry_run: bool,
) -> svm_result_t {
    debug!("`svm_exec_app` start");

    let bytes = std::slice::from_raw_parts(bytes.bytes, bytes.length as usize);

    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);

    if host_ctx.is_err() {
        todo!();
        // update_last_error(e);
        // error!("`svm_exec_app` returns `SVM_FAILURE`");
        // return svm_result_t::SVM_FAILURE;
    }

    let host_ctx = host_ctx.unwrap();
    let runtime = helpers::cast_to_runtime_mut(runtime);
    let state: Result<State, String> = state.into();

    if let Err(msg) = state {
        todo!();
    }

    let receipt = runtime.exec_app(bytes, &state.unwrap(), host_ctx, dry_run);
    todo!();

    //         let mut bytes = crate::receipt::encode_receipt(native_receipt);
    //         // returning encoded `Receipt` as `svm_byte_array`
    //         // should call later `svm_receipt_destroy`
    //         vec_to_svm_byte_array!(receipt, bytes);

    //         debug!("`svm_exec_app` returns `SVM_SUCCESS`");
    //         svm_result_t::SVM_SUCCESS
    //     }
    //     Err(_e) => {
    //         // update_last_error(e);
    //         error!("`svm_exec_app` returns `SVM_FAILURE`");
    //         svm_result_t::SVM_FAILURE
    //     }
    // }
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

/// Destroys the Runtime and its associated resources.
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
/// use svm_common::Address;
///
/// let mut host = std::ptr::null_mut();
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports) };
/// assert!(res.is_ok());
///
/// // destroy runtime
/// unsafe { svm_runtime_destroy(runtime); }
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_destroy(runtime: *mut c_void) {
    debug!("`svm_runtime_destroy`");

    let _ = Box::from_raw(runtime as *mut RuntimePtr);
}

/// Frees allocated imports resources.
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// // allocate imports
/// let count = 0;
/// let mut imports = std::ptr::null_mut();
/// let _res = unsafe { svm_imports_alloc(&mut imports, count) };
///
/// // destroy imports
/// unsafe { svm_imports_destroy(imports); }
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_imports_destroy(imports: *const c_void) {
    let _ = Box::from_raw(imports as *mut Vec<svm_import_t>);
}

/// Frees `svm_byte_array`
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// let bytes = svm_byte_array::default();
/// unsafe { svm_byte_array_destroy(bytes); }
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_byte_array_destroy(bytes: svm_byte_array) {
    let ptr = bytes.bytes as *mut u8;
    let length = bytes.length as usize;

    let _ = Vec::from_raw_parts(ptr, length, length);
}
