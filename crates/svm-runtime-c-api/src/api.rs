use std::{convert::TryFrom, ffi::c_void, path::Path, ptr::NonNull};

use log::{debug, error};

use svm_app::{default::DefaultSerializerTypes, types::HostCtx};
use svm_common::{Address, State};
use svm_runtime::{ctx::SvmCtx, gas::DefaultGasEstimator};

use crate::{
    helpers, raw_error, raw_utf8_error, raw_validate_error,
    receipt::{encode_app_receipt, encode_exec_receipt, encode_template_receipt},
    svm_byte_array, svm_import_func_sig_t, svm_import_func_t, svm_import_kind, svm_import_t,
    svm_import_value, svm_result_t, svm_value_type_array,
    testing::{self, ClientAppReceipt, ClientExecReceipt, ClientTemplateReceipt},
    RuntimePtr,
};

macro_rules! maybe_gas {
    ($gas_metering:expr, $gas_limit:expr) => {{
        use svm_runtime::gas::MaybeGas;

        if $gas_metering {
            MaybeGas::with($gas_limit)
        } else {
            MaybeGas::new()
        }
    }};
}

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
    error: *mut svm_byte_array,
) -> svm_result_t {
    let runtime = helpers::cast_to_runtime(runtime);

    match runtime.validate_template(bytes.into()) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => {
            error!("`svm_validate_template` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_app(
    runtime: *const c_void,
    bytes: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let runtime = helpers::cast_to_runtime(runtime);

    match runtime.validate_app(bytes.into()) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => {
            error!("`svm_validate_app` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
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
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let mut app_addr = svm_byte_array::default();
/// let tx_bytes = svm_byte_array::default();
/// let _res = unsafe { svm_validate_tx(&mut app_addr, runtime, tx_bytes, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_tx(
    app_addr: *mut svm_byte_array,
    runtime: *const c_void,
    bytes: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_validate_tx` start");

    let runtime = helpers::cast_to_runtime(runtime);

    match runtime.validate_tx(bytes.into()) {
        Ok(addr) => {
            // returning encoded `AppReceipt` as `svm_byte_array`.
            // should call later `svm_receipt_destroy`
            addr_to_svm_byte_array!(app_addr, addr.unwrap());

            debug!("`svm_validate_tx` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(e) => {
            error!("`svm_validate_tx` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
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
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe {
///   svm_import_func_build(
///     imports,
///     module_name,
///     import_name,
///     func,
///     params,
///     returns,
///     &mut error)
/// };
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
    error: *mut svm_byte_array,
) -> svm_result_t {
    let imports = &mut *(imports as *mut Vec<svm_import_t>);

    assert!(imports.len() < imports.capacity());

    let func = NonNull::new(func as *mut c_void);
    if func.is_none() {
        let s = String::from("`func` parameter must not be NULL");
        raw_error(s, error);

        return svm_result_t::SVM_FAILURE;
    }

    let func = svm_import_func_t {
        func: func.unwrap(),
        sig: svm_import_func_sig_t {
            params: params.into(),
            returns: returns.into(),
        },
    };

    let module_name = String::try_from(module_name);
    if module_name.is_err() {
        raw_utf8_error(module_name, error);
        return svm_result_t::SVM_FAILURE;
    }

    let import_name = String::try_from(import_name);
    if import_name.is_err() {
        raw_utf8_error(import_name, error);
        return svm_result_t::SVM_FAILURE;
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
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports, &mut error) };
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
    _error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_memory_runtime_create` start");

    let imports = helpers::cast_imports_to_wasmer_imports(imports);

    let kv = svm_common::from_raw_mut(kv);
    let mem_runtime = svm_runtime::testing::create_memory_runtime(host, kv, imports);

    let res = box_runtime!(runtime, mem_runtime);

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
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_runtime_create(&mut runtime, path, host, imports, &mut error) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    runtime: *mut *mut c_void,
    kv_path: svm_byte_array,
    host: *mut c_void,
    imports: *const c_void,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let kv_path: Result<String, std::string::FromUtf8Error> = String::try_from(kv_path);

    if kv_path.is_err() {
        raw_utf8_error(kv_path, error);
        return svm_result_t::SVM_FAILURE;
    }

    let kv_path = kv_path.unwrap();
    let imports = helpers::cast_imports_to_wasmer_imports(imports);

    let rocksdb_runtime = svm_runtime::create_rocksdb_runtime::<
        &Path,
        DefaultSerializerTypes,
        DefaultGasEstimator,
    >(host, Path::new(&kv_path), imports);

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
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports, &mut error) };
/// assert!(res.is_ok());
///
/// // deploy template
/// let mut receipt = svm_byte_array::default();
/// let author: svm_byte_array = Address::of("@author").into();
/// let host_ctx = svm_byte_array::default();
/// let template_bytes = svm_byte_array::default();
/// let gas_metering = false;
/// let gas_limit = 0;
/// let dry_run = false;
///
/// let res = unsafe {
///   svm_deploy_template(
///     &mut receipt,
///     runtime,
///     template_bytes,
///     author,
///     host_ctx,
///     gas_metering,
///     gas_limit,
///     dry_run,
///     &mut error)
/// };
///
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_deploy_template(
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    bytes: svm_byte_array,
    author: svm_byte_array,
    host_ctx: svm_byte_array,
    gas_metering: bool,
    gas_limit: u64,
    dry_run: bool,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_deploy_template` start`");

    let runtime = helpers::cast_to_runtime_mut(runtime);

    let author: Result<Address, String> = Address::try_from(author);

    if let Err(s) = author {
        raw_error(s, error);
        return svm_result_t::SVM_FAILURE;
    }

    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);
    if host_ctx.is_err() {
        let s = host_ctx.err().unwrap();
        raw_error(s, error);
        return svm_result_t::SVM_FAILURE;
    }

    let gas_limit = maybe_gas!(gas_metering, gas_limit);

    let rust_receipt = runtime.deploy_template(
        bytes.into(),
        &author.unwrap().into(),
        host_ctx.unwrap(),
        gas_limit,
        dry_run,
    );

    let mut receipt_bytes = encode_template_receipt(&rust_receipt);

    // returning encoded `TemplateReceipt` as `svm_byte_array`.
    // should call later `svm_receipt_destroy`
    vec_to_svm_byte_array!(receipt, receipt_bytes);

    debug!("`svm_exec_app` returns `SVM_SUCCESS`");

    svm_result_t::SVM_SUCCESS
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
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let mut app_receipt = svm_byte_array::default();
/// let mut init_state = svm_byte_array::default();
/// let creator = Address::of("@creator").into();
/// let host_ctx = svm_byte_array::default();
/// let app_bytes = svm_byte_array::default();
/// let gas_metering = false;
/// let gas_limit = 0;
/// let dry_run = false;
///
/// let _res = unsafe {
///   svm_spawn_app(
///     &mut app_receipt,
///     runtime,
///     app_bytes,
///     creator,
///     host_ctx,
///     gas_metering,
///     gas_limit,
///     dry_run,
///     &mut error)
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_spawn_app(
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    bytes: svm_byte_array,
    creator: svm_byte_array,
    host_ctx: svm_byte_array,
    gas_metering: bool,
    gas_limit: u64,
    dry_run: bool,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_spawn_app` start");

    let runtime = helpers::cast_to_runtime_mut(runtime);
    let creator: Result<Address, String> = Address::try_from(creator);

    if let Err(s) = creator {
        raw_error(s, error);
        return svm_result_t::SVM_FAILURE;
    }

    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);
    if host_ctx.is_err() {
        let s = host_ctx.err().unwrap();
        raw_error(s, error);
        return svm_result_t::SVM_FAILURE;
    }

    let gas_limit = maybe_gas!(gas_metering, gas_limit);

    let rust_receipt = runtime.spawn_app(
        bytes.into(),
        &creator.unwrap().into(),
        host_ctx.unwrap(),
        gas_limit,
        dry_run,
    );

    let mut receipt_bytes = encode_app_receipt(&rust_receipt);

    // returning encoded `AppReceipt` as `svm_byte_array`.
    // should call later `svm_receipt_destroy`
    vec_to_svm_byte_array!(receipt, receipt_bytes);

    debug!("`svm_spawn_app` returns `SVM_SUCCESS`");

    svm_result_t::SVM_SUCCESS
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
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let mut exec_receipt = svm_byte_array::default();
/// let tx_bytes = svm_byte_array::default();
/// let state = State::empty().into();
/// let host_ctx = svm_byte_array::default();
/// let gas_metering = false;
/// let dry_run = false;
/// let gas_limit = 0;
///
/// let _res = unsafe {
///   svm_exec_app(
///     &mut exec_receipt,
///     runtime,
///     tx_bytes,
///     state,
///     host_ctx,
///     gas_metering,
///     gas_limit,
///     dry_run,
///     &mut error)
/// };
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
    gas_metering: bool,
    gas_limit: u64,
    dry_run: bool,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_exec_app` start");

    let host_ctx = HostCtx::from_raw_parts(host_ctx.bytes, host_ctx.length);
    if host_ctx.is_err() {
        let s = host_ctx.err().unwrap();
        raw_error(s, error);
        return svm_result_t::SVM_FAILURE;
    }

    let host_ctx = host_ctx.unwrap();
    let runtime = helpers::cast_to_runtime_mut(runtime);
    let state: Result<State, String> = State::try_from(state);

    if let Err(msg) = state {
        raw_error(msg, error);
        return svm_result_t::SVM_FAILURE;
    }

    let gas_limit = maybe_gas!(gas_metering, gas_limit);

    let rust_receipt =
        runtime.exec_app(bytes.into(), &state.unwrap(), host_ctx, gas_limit, dry_run);
    let mut receipt_bytes = encode_exec_receipt(&rust_receipt);

    // returning encoded `ExecReceipt` as `svm_byte_array`.
    // should call later `svm_receipt_destroy`
    vec_to_svm_byte_array!(receipt, receipt_bytes);

    debug!("`svm_exec_app` returns `SVM_SUCCESS`");

    svm_result_t::SVM_SUCCESS
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
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, host, imports, &mut error) };
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

/// Receipts helpers

/// In order to spare the SVM client the implementation of the `Receipt`(s) raw decoding the receipts helpers
/// can fetch one field each. This functionality should be useful for writing tests when using client code that interfaces with
/// SVM FFI interface.
///
/// Each helper methods returns `svm_result_t`.
/// When `svm_result_t` equals `SVM_SUCCESS` is means that the field extraction succeeded.
/// Otherwise, it signals that the field can't be extracted out of the receipt.
///
/// For example, if the `svm_deploy_template` failed to deploy the template (it may happen for many reason, one is having invalid wasm code),
/// then calling `svm_template_receipt_addr` should return `SVM_FAILURE` since there is no template `Address` to extract.
/// The error will be returned via the `error` parameter.

/// `Deploy-Template` Receipt helpers
///  -------------------------------------------------------

/// Extracts the deploy-template `Address` into the `template_addr` parameter. (useful for tests).
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_template_receipt_addr(
    template_addr: *mut svm_byte_array,
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_template_receipt(receipt.into());

    match client_receipt {
        ClientTemplateReceipt::Success { addr, .. } => {
            addr_to_svm_byte_array!(template_addr, addr.unwrap());
            svm_result_t::SVM_SUCCESS
        }
        ClientTemplateReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Extracts the `gas_used` for the deploy-template.
/// When deploying succeeded returns `SVM_SUCCESS`, returns the amount of gas used via `gas_used` parameter.
/// Othewrise, returns `SVM_FAILURE` and the error message via the `error` parameter.
///
/// It's up for the Host to decide the gas fee to for a failed deploy.
/// (usually the strategy will be to fine with the `gas_limit` of the failed transaction).
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_template_receipt_gas(
    gas_used: *mut u64,
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_template_receipt(receipt.into());

    match client_receipt {
        ClientTemplateReceipt::Success { gas_used: gas, .. } => {
            *gas_used = gas;
            svm_result_t::SVM_SUCCESS
        }
        ClientTemplateReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// `Spawn-App` Receipt helpers
///  -------------------------------------------------------

/// Extracts whether the `spawn-app` transaction succeeded.
/// If it succeeded, returns `SVM_SUCCESS`,
/// Otherwise returns `SVM_FAILURE` and the error message via `error` parameter.
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_app_receipt_status(
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_app_receipt(receipt.into());

    match client_receipt {
        ClientAppReceipt::Success { .. } => svm_result_t::SVM_SUCCESS,
        ClientAppReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Extracts the spawned-app `Address`.
/// When spawning succeeds returns `SVM_SUCCESS` and the `Address` via `app_addr` parameter.
/// Otherise, returns `SVM_FAILURE` and the error message via the `error` parameter.
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_app_receipt_addr(
    app_addr: *mut svm_byte_array,
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_app_receipt(receipt.into());

    match client_receipt {
        ClientAppReceipt::Success { addr, .. } => {
            addr_to_svm_byte_array!(app_addr, addr.unwrap());
            svm_result_t::SVM_SUCCESS
        }
        ClientAppReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Extracts the `gas_used` for spawned-app (including running its constructor).
/// When spawn succeeded returns `SVM_SUCCESS`, returns the amount of gas used via `gas_used` parameter.
/// Othewrise, returns `SVM_FAILURE` and the error message via the `error` parameter.
///
/// It's up for the Host to decide the gas fee to for failed spawning.
/// (usually the strategy will be to fine with the `gas_limit` of the failed transaction).
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_app_receipt_gas(
    gas_used: *mut u64,
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_app_receipt(receipt.into());

    match client_receipt {
        ClientAppReceipt::Success { gas_used: gas, .. } => {
            *gas_used = gas;
            svm_result_t::SVM_SUCCESS
        }
        ClientAppReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// `Exec-App` Receipt helpers
///  -------------------------------------------------------

/// Extracts the spawned-app initial `State`.
/// When spawning succeeds returns `SVM_SUCCESS` and the initial `State` via `state` parameter.
/// Otherise, returns `SVM_FAILURE` and the error message via the `error` parameter.
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_app_receipt_state(
    state: *mut svm_byte_array,
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_app_receipt(receipt.into());

    match client_receipt {
        ClientAppReceipt::Success { init_state, .. } => {
            state_to_svm_byte_array!(state, init_state);
            svm_result_t::SVM_SUCCESS
        }
        ClientAppReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Extracts whether the `exec-app` transaction succeeded.
/// If it succeeded, returns `SVM_SUCCESS`,
/// Otherwise returns `SVM_FAILURE` and the error message via `error` parameter.
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_exec_receipt_status(
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_exec_receipt(receipt.into());

    match client_receipt {
        ClientExecReceipt::Success { .. } => svm_result_t::SVM_SUCCESS,
        ClientExecReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Extracts the executed transaction new `State`.
/// When transaction succeeded returns `SVM_SUCCESS` and the new `State` via `state` parameter.
/// Othewrise, returns `SVM_FAILURE` and the error message via the `error` parameter.
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_exec_receipt_state(
    state: *mut svm_byte_array,
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_exec_receipt(receipt.into());

    match client_receipt {
        ClientExecReceipt::Success { new_state, .. } => {
            state_to_svm_byte_array!(state, new_state);
            svm_result_t::SVM_SUCCESS
        }
        ClientExecReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Extracts the executed transaction `gas_used`.
/// When transaction succeeded returns `SVM_SUCCESS`, returns the amount of gas used via `gas_used` parameter.
/// Othewrise, returns `SVM_FAILURE` and the error message via the `error` parameter.
///
/// It's up for the Host to decide the gas fee to for failed transactions.
/// (usually the strategy will be to fine with the `gas_limit` of the failed transaction).
///
/// # Panics
///
/// Panics the `receipt` input is invalid.
///
#[no_mangle]
pub unsafe extern "C" fn svm_exec_receipt_gas(
    gas_used: *mut u64,
    receipt: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let client_receipt = testing::decode_exec_receipt(receipt.into());

    match client_receipt {
        ClientExecReceipt::Success { gas_used: gas, .. } => {
            *gas_used = gas;
            svm_result_t::SVM_SUCCESS
        }
        ClientExecReceipt::Failure { error: err_str } => {
            raw_error(err_str, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn svm_estimate_deploy_template(
    _estimate: *mut u64,
    runtime: *mut c_void,
    _bytes: svm_byte_array,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    let _runtime = helpers::cast_to_runtime_mut(runtime);

    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn svm_estimate_spawn_app(
    _estimate: *mut u64,
    _runtime: *mut c_void,
    _bytes: svm_byte_array,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn svm_estimate_exec_app(
    _estimate: *mut u64,
    _runtime: *mut c_void,
    _bytes: svm_byte_array,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    todo!()
}
