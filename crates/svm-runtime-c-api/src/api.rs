use std::cell::RefCell;
use std::convert::TryFrom;
use std::ffi::c_void;
use std::io;
use std::ptr::NonNull;
use std::rc::Rc;

#[cfg(feature = "default-rocksdb")]
use std::path::Path;

#[cfg(feature = "default-rocksdb")]
use svm_runtime::{DefaultGasEstimator, DefaultSerializers};

#[cfg(feature = "default-rocksdb")]
use svm_storage::kv::ExternKV;

use log::{debug, error};

use svm_codec::receipt;
use svm_ffi::{svm_byte_array, svm_func_callback_t, svm_resource_iter_t, svm_resource_t, tracking};
use svm_runtime::{ExternImport, Runtime, RuntimePtr};
use svm_storage::kv::StatefulKV;
use svm_types::{Address, State, Type, WasmType};

use crate::{raw_error, raw_io_error, raw_utf8_error, raw_validate_error, svm_result_t};

static KV_TYPE: Type = Type::Str("key-value store");
static VALIDATE_TX_APP_ADDR_TYPE: Type = Type::Str("svm_validate_tx app_addr");
static DEPLOY_TEMPLATE_RECEIPT_TYPE: Type = Type::Str("deploy-template receipt");
static SPAWN_APP_RECEIPT_TYPE: Type = Type::Str("spawn-app receipt");
static EXEC_APP_RECEIPT_TYPE: Type = Type::Str("exec-app receipt");

macro_rules! maybe_gas {
    ($gas_metering:expr, $gas_limit:expr) => {{
        use svm_types::gas::MaybeGas;

        if $gas_metering {
            MaybeGas::with($gas_limit)
        } else {
            MaybeGas::new()
        }
    }};
}

macro_rules! addr_to_svm_byte_array {
    ($ty:expr, $raw_byte_array:expr, $addr:expr) => {{
        let (ptr, len, cap) = $addr.into_raw_parts();

        to_svm_byte_array!($ty, $raw_byte_array, ptr, len, cap);
    }};
}

macro_rules! vec_to_svm_byte_array {
    ($ty:expr, $raw_byte_array:expr, $vec:expr) => {{
        let (ptr, len, cap) = $vec.into_raw_parts();

        to_svm_byte_array!($ty, $raw_byte_array, ptr, len, cap);
    }};
}

macro_rules! to_svm_byte_array {
    ($ty:expr, $raw_byte_array:expr, $ptr:expr, $len:expr, $cap:expr) => {{
        let bytes: &mut svm_byte_array = &mut *$raw_byte_array;

        tracking::increment_live($ty);

        bytes.bytes = $ptr;
        bytes.length = $len as u32;
        bytes.capacity = $cap as u32;
        bytes.type_id = tracking::interned_type($ty);
    }};
}

/// Validates syntactically a raw `deploy template` transaction.
///
/// Should be called while the transaction is in the `mempool` of the Host.
/// In case the transaction isn't valid - the transaction should be discarded.
///
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address;
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let bytes = svm_byte_array::default();
/// let _res = unsafe { svm_validate_template(runtime, bytes, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_template(
    runtime: *mut c_void,
    bytes: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let runtime: &mut Box<dyn Runtime> = runtime.into();

    match runtime.validate_template(bytes.into()) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => {
            error!("`svm_validate_template` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Validates syntactically a raw `spawn app` transaction.
///
/// Should be called while the transaction is in the `mempool` of the Host.
/// In case the transaction isn't valid - the transaction should be discarded.
///
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address;
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let bytes = svm_byte_array::default();
/// let _res = unsafe { svm_validate_app(runtime, bytes, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_app(
    runtime: *mut c_void,
    bytes: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let runtime: &mut Box<dyn Runtime> = runtime.into();

    match runtime.validate_app(bytes.into()) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => {
            error!("`svm_validate_app` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Validates syntactically a raw `execute app` transaction.
/// Returns the `App` address that appears in the transaction.
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address;
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let mut app_addr = svm_byte_array::default();
/// let bytes = svm_byte_array::default();
/// let _res = unsafe { svm_validate_tx(&mut app_addr, runtime, bytes, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_tx(
    app_addr: *mut svm_byte_array,
    runtime: *mut c_void,
    bytes: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_validate_tx` start");

    let runtime: &mut Box<dyn Runtime> = runtime.into();

    match runtime.validate_tx(bytes.into()) {
        Ok(tx) => {
            // returning encoded `AppReceipt` as `svm_byte_array`.
            // should call later `svm_receipt_destroy`
            addr_to_svm_byte_array!(VALIDATE_TX_APP_ADDR_TYPE, app_addr, tx.app.unwrap());

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
/// See `svm_imports_destroy` for freeing the imports.
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
    let vec: Vec<ExternImport> = Vec::with_capacity(count as usize);
    let ty = Type::of::<Vec<ExternImport>>();

    *imports = svm_ffi::into_raw(ty, vec);

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
/// use svm_ffi::{svm_env_t, svm_func_callback_t, svm_byte_array};
/// use svm_types::{WasmType, Type};
///
/// unsafe extern "C" fn host_func(
///   env:     *mut svm_env_t,
///   args:    *const svm_byte_array,
///   results: *mut svm_byte_array
/// ) -> *mut svm_byte_array {
///   // ...
///   return std::ptr::null_mut()
/// }
///
/// #[repr(C)]
/// struct function_id(u32);
///
/// // allocate one import
/// let mut imports = testing::imports_alloc(1);
///
/// let namespace_ty = Type::Str("import ns");
/// let name_ty = Type::Str("import name");
/// let params_ty = Type::Str("import params");
/// let returns_ty = Type::Str("import returns");
/// let host_env_ty = Type::Str("host env");
///
/// let namespace: svm_byte_array = (namespace_ty, String::from("env")).into();
/// let import_name: svm_byte_array = (name_ty, String::from("foo")).into();
/// let params: svm_byte_array = (params_ty, Vec::<WasmType>::new()).into();
/// let returns: svm_byte_array = (returns_ty,Vec::<WasmType>::new()).into();
/// let mut error = svm_byte_array::default();
///
/// let host_env = svm_ffi::into_raw(host_env_ty, function_id(0));
///
/// let res = unsafe {
///   svm_import_func_new(
///     imports,
///     namespace.clone(),
///     import_name.clone(),
///     host_func,
///     host_env,
///     params.clone(),
///     returns.clone(),
///     &mut error)
/// };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_import_func_new(
    imports: *mut c_void,
    namespace: svm_byte_array,
    import_name: svm_byte_array,
    func: svm_func_callback_t,
    host_env: *const c_void,
    params: svm_byte_array,
    returns: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let imports = svm_ffi::as_mut::<Vec<ExternImport>>(imports);

    assert!(imports.len() < imports.capacity());

    let host_env = NonNull::new(host_env as *mut c_void);
    if host_env.is_none() {
        let s = String::from("`host_env` parameter must not be NULL");
        raw_error(s, error);

        return svm_result_t::SVM_FAILURE;
    }

    let host_env: *const c_void = host_env.unwrap().as_ptr();

    let params: Result<Vec<WasmType>, io::Error> = Vec::try_from(params);
    if let Err(e) = params {
        raw_io_error(e, error);
        return svm_result_t::SVM_FAILURE;
    }

    let returns: Result<Vec<WasmType>, io::Error> = Vec::try_from(returns);
    if let Err(e) = returns {
        raw_io_error(e, error);
        return svm_result_t::SVM_FAILURE;
    }

    let import_name = String::try_from(import_name);
    if import_name.is_err() {
        raw_utf8_error(import_name, error);
        return svm_result_t::SVM_FAILURE;
    }

    let namespace = String::try_from(namespace);
    if namespace.is_err() {
        raw_utf8_error(namespace, error);
        return svm_result_t::SVM_FAILURE;
    }

    let import = ExternImport::new(
        import_name.unwrap(),
        namespace.unwrap(),
        params.unwrap(),
        returns.unwrap(),
        func,
        host_env,
    );

    imports.push(import);

    svm_result_t::SVM_SUCCESS
}

macro_rules! box_runtime {
    ($raw_runtime:expr, $runtime:expr) => {{
        let runtime_ptr = RuntimePtr::new(Box::new($runtime));

        // `svm_runtime_destroy` should be called later for freeing memory.
        *$raw_runtime = RuntimePtr::into_raw(runtime_ptr);

        svm_result_t::SVM_SUCCESS
    }};
}

/// Creates a new in-memory key-value client.
/// Returns a raw pointer to allocated kv-store via input parameter `kv`.
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_state_kv_create(kv: *mut *mut c_void) -> svm_result_t {
    let state_kv = svm_runtime::testing::memory_state_kv_init();

    *kv = svm_ffi::into_raw(KV_TYPE, state_kv);

    svm_result_t::SVM_SUCCESS
}

/// Creates a new FFI key-value client.
/// Returns a raw pointer to allocated kv-store via input parameter `kv`.
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// unsafe extern "C" fn get(key_ptr: *const u8, key_len: u32, value_ptr: *mut u8, value_len: *mut u32) {}
/// unsafe extern "C" fn set(key_ptr: *const u8, key_len: u32, value_ptr: *const u8, value_len: u32) {}
/// unsafe extern "C" fn discard() {}
/// unsafe extern "C" fn checkpoint(state: *mut u8) {}
/// unsafe extern "C" fn head(state: *mut u8) {}
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe {
///   svm_ffi_state_kv_create(
///     &mut kv,
///     get,
///     set,
///     discard,
///     checkpoint,
///     head)
/// };
//
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-rocksdb")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_ffi_state_kv_create(
    state_kv: *mut *mut c_void,
    get_fn: unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u32),
    set_fn: unsafe extern "C" fn(*const u8, u32, *const u8, u32),
    discard_fn: unsafe extern "C" fn(),
    checkpoint_fn: unsafe extern "C" fn(*mut u8),
    head_fn: unsafe extern "C" fn(*mut u8),
) -> svm_result_t {
    let ffi_kv = ExternKV {
        get_fn,
        set_fn,
        discard_fn,
        checkpoint_fn,
        head_fn,
    };

    let ffi_kv: Rc<RefCell<dyn StatefulKV>> = Rc::new(RefCell::new(ffi_kv));

    *state_kv = svm_ffi::into_raw(KV_TYPE, ffi_kv);

    svm_result_t::SVM_SUCCESS
}

/// Frees an in-memory key-value.
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let res = unsafe { svm_state_kv_destroy(kv) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_state_kv_destroy(kv: *mut c_void) -> svm_result_t {
    let kv: &mut Rc<RefCell<dyn StatefulKV>> = svm_ffi::as_mut(kv);

    let _ = svm_ffi::from_raw(KV_TYPE, kv);

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
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
/// let mut imports = testing::imports_alloc(0);
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, imports, &mut error) };
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-memory")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(
    runtime: *mut *mut c_void,
    state_kv: *mut c_void,
    imports: *mut c_void,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_memory_runtime_create` start");

    let imports = svm_ffi::as_mut::<Vec<ExternImport>>(imports);
    let state_kv = svm_ffi::as_mut(state_kv);
    let mem_runtime = svm_runtime::testing::create_memory_runtime(state_kv, imports);

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
/// use svm_types::Type;
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let ty = Type::Str("path");
/// let path = String::from("path goes here");

/// let path: svm_byte_array = (ty, path).into();
/// let mut imports = testing::imports_alloc(0);
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_runtime_create(&mut runtime, path, imports, &mut error) };
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-rocksdb")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    runtime: *mut *mut c_void,
    state_kv: *mut c_void,
    kv_path: svm_byte_array,
    imports: *mut c_void,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let kv_path: Result<String, std::string::FromUtf8Error> = String::try_from(kv_path);

    if kv_path.is_err() {
        raw_utf8_error(kv_path, error);
        return svm_result_t::SVM_FAILURE;
    }

    let kv_path = kv_path.unwrap();
    let imports = svm_ffi::as_mut::<Vec<ExternImport>>(imports);
    let state_kv = svm_ffi::as_mut(state_kv);

    let rocksdb_runtime = svm_runtime::create_rocksdb_runtime::<
        &Path,
        DefaultSerializers,
        DefaultGasEstimator,
    >(&state_kv, Path::new(&kv_path), imports);

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
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::{Address, Type};
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
/// let mut state_kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut state_kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, imports, &mut error) };
/// assert!(res.is_ok());
///
/// // deploy template
/// let mut receipt = svm_byte_array::default();
/// let ty = Type::Str("author");
/// let author: svm_byte_array = (ty, Address::of("@author")).into();
/// let template_bytes = svm_byte_array::default();
/// let gas_metering = false;
/// let gas_limit = 0;
///
/// let res = unsafe {
///   svm_deploy_template(
///     &mut receipt,
///     runtime,
///     template_bytes,
///     author,
///     gas_metering,
///     gas_limit,
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
    gas_metering: bool,
    gas_limit: u64,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_deploy_template` start`");

    let runtime: &mut Box<dyn Runtime> = runtime.into();

    let author: Result<Address, String> = Address::try_from(author);

    if let Err(s) = author {
        raw_error(s, error);
        return svm_result_t::SVM_FAILURE;
    }

    let gas_limit = maybe_gas!(gas_metering, gas_limit);
    let rust_receipt = runtime.deploy_template(bytes.into(), &author.unwrap().into(), gas_limit);
    let receipt_bytes = receipt::encode_template_receipt(&rust_receipt);

    // returning encoded `TemplateReceipt` as `svm_byte_array`.
    // should call later `svm_receipt_destroy`
    vec_to_svm_byte_array!(DEPLOY_TEMPLATE_RECEIPT_TYPE, receipt, receipt_bytes);

    debug!("`svm_deploy_template` returns `SVM_SUCCESS`");

    svm_result_t::SVM_SUCCESS
}

/// Spawns a new App.
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::{Address, Type};
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
///
/// let mut state_kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut state_kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let mut app_receipt = svm_byte_array::default();
/// let mut init_state = svm_byte_array::default();
///
/// let spawner_ty = Type::Str("spawner");
/// let spawner: svm_byte_array = (spawner_ty, Address::of("@spawner")).into();
/// let app_bytes = svm_byte_array::default();
/// let gas_metering = false;
/// let gas_limit = 0;
///
/// let _res = unsafe {
///   svm_spawn_app(
///     &mut app_receipt,
///     runtime,
///     app_bytes,
///     spawner,
///     gas_metering,
///     gas_limit,
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
    spawner: svm_byte_array,
    gas_metering: bool,
    gas_limit: u64,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_spawn_app` start");

    let runtime: &mut Box<dyn Runtime> = runtime.into();
    let spawner: Result<Address, String> = Address::try_from(spawner);

    if let Err(s) = spawner {
        raw_error(s, error);
        return svm_result_t::SVM_FAILURE;
    }

    let gas_limit = maybe_gas!(gas_metering, gas_limit);
    let rust_receipt = runtime.spawn_app(bytes.into(), &spawner.unwrap().into(), gas_limit);
    let receipt_bytes = receipt::encode_app_receipt(&rust_receipt);

    // returning encoded `AppReceipt` as `svm_byte_array`.
    // should call later `svm_receipt_destroy`
    vec_to_svm_byte_array!(SPAWN_APP_RECEIPT_TYPE, receipt, receipt_bytes);

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
///
/// use svm_types::{State, Address, Type};
/// use svm_ffi::svm_byte_array;
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
///
/// let mut state_kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut state_kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, imports, &mut error) };
/// assert!(res.is_ok());
///
/// let mut exec_receipt = svm_byte_array::default();
/// let bytes = svm_byte_array::default();
/// let ty = Type::of::<State>();
/// let state = (ty, State::zeros()).into();
/// let gas_metering = false;
/// let gas_limit = 0;
///
/// let _res = unsafe {
///   svm_exec_app(
///     &mut exec_receipt,
///     runtime,
///     bytes,
///     state,
///     gas_metering,
///     gas_limit,
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
    gas_metering: bool,
    gas_limit: u64,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_exec_app` start");

    let runtime: &mut Box<dyn Runtime> = runtime.into();
    let state: Result<State, String> = State::try_from(state);

    if let Err(msg) = state {
        raw_error(msg, error);
        return svm_result_t::SVM_FAILURE;
    }

    let gas_limit = maybe_gas!(gas_metering, gas_limit);

    let tx = runtime.validate_tx(bytes.into()).unwrap();
    let rust_receipt = runtime.exec_tx(&tx, &state.unwrap(), gas_limit);
    let receipt_bytes = receipt::encode_exec_receipt(&rust_receipt);

    // returning encoded `ExecReceipt` as `svm_byte_array`.
    // should call later `svm_receipt_destroy`
    vec_to_svm_byte_array!(EXEC_APP_RECEIPT_TYPE, receipt, receipt_bytes);

    debug!("`svm_exec_app` returns `SVM_SUCCESS`");

    svm_result_t::SVM_SUCCESS
}

/// Returns the total live manually-managed resources.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_total_live_resources() -> i32 {
    tracking::total_live()
}

/// Initializes a new iterator over the manually-managed resources
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_new() -> *mut c_void {
    let ty = svm_ffi::SVM_RESOURCES_ITER_TYPE;
    let snapshot = tracking::take_snapshot();

    svm_ffi::into_raw(ty, snapshot)
}

/// Destroys the manually-managed resources iterator
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_destroy(iter: *mut c_void) {
    let ty = svm_ffi::SVM_RESOURCES_ITER_TYPE;
    let _ = svm_ffi::from_raw(ty, iter);
}

/// Returns the next manually-managed resource.
/// If there is no resource to return, returns `NULL`
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_next(iter: *mut c_void) -> *mut svm_resource_t {
    let iter = svm_ffi::as_mut::<svm_resource_iter_t>(iter);

    match iter.next() {
        None => std::ptr::null_mut(),
        Some(resource) => {
            let ty = svm_ffi::SVM_RESOURCE_TYPE;
            let ptr = svm_ffi::into_raw(ty, resource);

            svm_ffi::as_mut::<svm_resource_t>(ptr)
        }
    }
}

/// Destroy the resource
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_destroy(resource: *mut svm_resource_t) {
    let _ = svm_ffi::from_raw(svm_ffi::SVM_RESOURCE_TYPE, resource);
}

/// Given a type in an interned form, returns its textual name
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_type_name_resolve(ty: usize) -> *mut svm_byte_array {
    match tracking::interned_type_rev(ty) {
        Some(ty) => {
            let ty = format!("{}", ty);
            let ty: svm_byte_array = (svm_ffi::SVM_RESOURCE_NAME_TYPE, ty).into();

            let ptr = svm_ffi::into_raw(svm_ffi::SVM_RESOURCE_NAME_PTR_TYPE, ty);
            ptr as _
        }
        None => std::ptr::null_mut(),
    }
}

/// Destorys a resource holding a type textual name
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_type_name_destroy(ptr: *mut svm_byte_array) {
    let ptr = svm_ffi::from_raw(svm_ffi::SVM_RESOURCE_NAME_PTR_TYPE, ptr);

    svm_byte_array_destroy(ptr)
}

/// Destroys the Runtime and its associated resources.
///
/// # Example
///
/// ```rust, no_run
/// use svm_runtime_c_api::*;
///
/// use svm_types::Address;
/// use svm_ffi::svm_byte_array;
///
/// // allocate imports
/// let mut imports = testing::imports_alloc(0);
///
/// // create runtime
///
/// let mut state_kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut state_kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, imports, &mut error) };
/// assert!(res.is_ok());
///
/// // destroy runtime
/// unsafe { svm_runtime_destroy(runtime); }
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_destroy(runtime: *mut c_void) {
    let _ = RuntimePtr::from_raw(runtime);
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
pub unsafe extern "C" fn svm_imports_destroy(imports: *mut c_void) {
    let imports = svm_ffi::as_mut::<Vec<ExternImport>>(imports);
    let ty = Type::of::<Vec<ExternImport>>();

    let _ = svm_ffi::from_raw::<Vec<ExternImport>>(ty, imports);
}

/// Frees `svm_byte_array`
///
/// # Example
///
/// ```rust
/// use svm_runtime_c_api::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let bytes = svm_byte_array::default();
/// unsafe { svm_byte_array_destroy(bytes); }
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_byte_array_destroy(bytes: svm_byte_array) {
    bytes.destroy()
}

/// Allocates a new error. Its context is a clone of the data given by parameter `msg`.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_wasm_error_create(msg: svm_byte_array) -> *mut svm_byte_array {
    let msg: &[u8] = msg.into();
    let bytes = msg.to_vec();

    let err: svm_byte_array = (svm_ffi::SVM_WASM_ERROR_TYPE, bytes).into();

    let ty = svm_ffi::SVM_WASM_ERROR_TYPE_PTR;
    let err = svm_ffi::into_raw(ty, err);

    svm_ffi::as_mut(err)
}
