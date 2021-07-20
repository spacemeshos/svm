#![allow(unused)]

use log::{debug, error};

use std::cell::RefCell;
use std::convert::TryFrom;
use std::ffi::c_void;
use std::rc::Rc;

#[cfg(feature = "default-rocksdb")]
use std::path::Path;

use svm_codec::receipt;
use svm_ffi::{svm_byte_array, svm_resource_iter_t, svm_resource_t, tracking};
use svm_runtime::{Runtime, RuntimePtr};
use svm_storage::kv::StatefulKV;
use svm_types::{Address, Gas, State, Type};

#[cfg(feature = "default-rocksdb")]
use crate::raw_utf8_error;

use crate::{raw_error, raw_validate_error, svm_result_t};

static ENVELOPE_TYPE: Type = Type::Str("Tx Envelope");
static CONTEXT_TYPE: Type = Type::Str("Tx Context");
static KV_TYPE: Type = Type::Str("Key-Value Store");
static VALIDATE_CALL_TARGET_TYPE: Type = Type::Str("validate_call Target");
static _DEPLOY_RECEIPT_TYPE: Type = Type::Str("Deploy Receipt");
static _SPAWN_RECEIPT_TYPE: Type = Type::Str("Spawn Receipt");
static _CALL_RECEIPT_TYPE: Type = Type::Str("Call Receipt");

#[inline]
fn maybe_gas(gas_enabled: bool, gas_limit: u64) -> Gas {
    if gas_enabled {
        Gas::with(gas_limit)
    } else {
        Gas::new()
    }
}

#[inline]
unsafe fn data_to_svm_byte_array(ty: Type, byte_array: *mut svm_byte_array, data: Vec<u8>) {
    let (ptr, len, cap) = data.into_raw_parts();
    let bytes: &mut svm_byte_array = &mut *byte_array;

    tracking::increment_live(ty);

    bytes.bytes = ptr;
    bytes.length = len as u32;
    bytes.capacity = cap as u32;
    bytes.type_id = tracking::interned_type(ty);
}

/// Allocates `svm_byte_array` of `size` bytes, destined to be used for passing a binary [`Envelope`].
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_envelope_alloc(size: u32) -> svm_byte_array {
    svm_byte_array::new(size as usize, ENVELOPE_TYPE)
}

/// Allocates `svm_byte_array` of `size` bytes, destined to be used for passing a binary [`Context`].
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_context_alloc(size: u32) -> svm_byte_array {
    svm_byte_array::new(size as usize, CONTEXT_TYPE)
}

/// Validates syntactically a raw `deploy template` transaction.
///
/// Should be called while the transaction is in the `mempool` of the Host.
/// In case the transaction isn't valid - the transaction should be discarded.
///
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address;
///
/// // Create runtime
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, &mut error) };
/// assert!(res.is_ok());
///
/// let bytes = svm_byte_array::default();
/// let _res = unsafe { svm_validate_deploy(runtime, bytes, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_deploy(
    runtime: *mut c_void,
    _envelope: svm_byte_array,
    message: svm_byte_array,
    _context: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let runtime: &mut Box<dyn Runtime> = runtime.into();

    match runtime.validate_deploy(message.as_bytes()) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => {
            error!("`svm_validate_template` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Validates syntactically a binary `Spawn Account` transaction.
///
/// Should be called while the transaction is in the `mempool` of the Host.
/// In case the transaction isn't valid - the transaction should be discarded.
///
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address;
///
/// // create runtime
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, &mut error) };
/// assert!(res.is_ok());
///
/// let bytes = svm_byte_array::default();
/// let _res = unsafe { svm_validate_spawn(runtime, bytes, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_spawn(
    runtime: *mut c_void,
    _envelope: svm_byte_array,
    message: svm_byte_array,
    _context: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let runtime: &mut Box<dyn Runtime> = runtime.into();

    match runtime.validate_spawn(message.as_bytes()) {
        Ok(()) => svm_result_t::SVM_SUCCESS,
        Err(e) => {
            error!("`svm_validate_spawn` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

/// Validates syntactically a binary `Call Account` transaction.
/// Returns the `Target Address` that appears in the transaction.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address;
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
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, &mut error) };
/// assert!(res.is_ok());
///
/// let mut target_addr = svm_byte_array::default();
/// let bytes = svm_byte_array::default();
/// let _res = unsafe { svm_validate_call(&mut target_addr, runtime, bytes, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_call(
    target: *mut svm_byte_array,
    runtime: *mut c_void,
    _envelope: svm_byte_array,
    message: svm_byte_array,
    _context: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_validate_call` start");

    let runtime: &mut Box<dyn Runtime> = runtime.into();

    match runtime.validate_call(message.as_bytes()) {
        Ok(tx) => {
            // Returns `target Address` that appears in `bytes`.
            //
            // # Notes
            //
            // Should call later `svm_receipt_destroy`
            data_to_svm_byte_array(
                VALIDATE_CALL_TARGET_TYPE,
                target,
                tx.target.unwrap().as_slice().to_vec(),
            );

            debug!("`svm_validate_call` returns `SVM_SUCCESS`");
            svm_result_t::SVM_SUCCESS
        }
        Err(e) => {
            error!("`svm_validate_call` returns `SVM_FAILURE`");
            raw_validate_error(&e, error);
            svm_result_t::SVM_FAILURE
        }
    }
}

macro_rules! box_runtime {
    ($raw_runtime:expr, $runtime:expr) => {{
        let runtime_ptr = RuntimePtr::new(Box::new($runtime));

        // # Notes
        //
        // `svm_runtime_destroy` should be called later for freeing memory.
        *$raw_runtime = RuntimePtr::into_raw(runtime_ptr);

        svm_result_t::SVM_SUCCESS
    }};
}

/// Creates a new in-memory key-value client.
/// Returns a raw pointer to allocated kv-store via input parameter `kv`.
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
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

/// Frees an in-memory key-value.
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
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
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let mut kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut kv) };
/// assert!(res.is_ok());
///
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, kv, &mut error) };
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-memory")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(
    runtime: *mut *mut c_void,
    state_kv: *mut c_void,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_memory_runtime_create` start");

    let state_kv = svm_ffi::as_mut(state_kv);
    let mem_runtime = svm_runtime::testing::create_memory_runtime(state_kv);

    let res = box_runtime!(runtime, mem_runtime);

    debug!("`svm_memory_runtime_create` end");

    res
}

/// Creates a new SVM Runtime instance.
/// Returns it via the `runtime` parameter.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_types::Type;
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
/// let mut state_kv = std::ptr::null_mut();
///
/// let ty = Type::Str("path");
/// let kv_path = String::from("path for SVM internal db goes here");

/// let kv_path: svm_byte_array = (ty, kv_path).into();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_runtime_create(&mut runtime, state_kv, kv_path, &mut error) };
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
    error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let kv_path: Result<String, std::string::FromUtf8Error> = String::try_from(kv_path);

    if kv_path.is_err() {
        raw_utf8_error(kv_path, error);
        return svm_result_t::SVM_FAILURE;
    }

    let kv_path = kv_path.unwrap();
    let state_kv = svm_ffi::as_mut(state_kv);

    let rocksdb_runtime = svm_runtime::create_rocksdb_runtime(&state_kv, &Path::new(&kv_path));

    let res = box_runtime!(runtime, rocksdb_runtime);

    debug!("`svm_runtime_create` end");

    res
}

/// Deploys a `Template`
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address;
///
/// // create runtime
/// let mut state_kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut state_kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, &mut error) };
/// assert!(res.is_ok());
///
/// // deploy template
/// let mut receipt = svm_byte_array::default();
/// let envelope = svm_byte_array::default();
/// let message = svm_byte_array::default();
/// let context = svm_byte_array::default();
/// let gas_enabled = false;
///
/// let res = unsafe {
///   svm_deploy(
///     &mut receipt,
///     runtime,
///     envelope,
///     message,
///     context,
///     gas_enabled,
///     &mut error)
/// };
///
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_deploy(
    _receipt: *mut svm_byte_array,
    _runtime: *mut c_void,
    _envelope: svm_byte_array,
    _message: svm_byte_array,
    _context: svm_byte_array,
    _gas_enabled: bool,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_deploy` start`");

    todo!("extract `gas_limit`, `deployer`");

    // let runtime: &mut Box<dyn Runtime> = runtime.into();

    // let deployer: Result<Address, String> = Address::try_from(deployer);

    // if let Err(s) = deployer {
    //     raw_error(s, error);
    //     return svm_result_t::SVM_FAILURE;
    // }

    // let gas_limit = maybe_gas(gas_enabled, gas_limit);
    // let rust_receipt = runtime.deploy(message.as_bytes(), &deployer.unwrap().into(), gas_limit);
    // let receipt_bytes = receipt::encode_deploy(&rust_receipt);

    // // returning encoded `TemplateReceipt` as `svm_byte_array`.
    // // should call later `svm_receipt_destroy`
    // data_to_svm_byte_array(DEPLOY_RECEIPT_TYPE, receipt, receipt_bytes);

    // debug!("`svm_deploy` returns `SVM_SUCCESS`");

    // svm_result_t::SVM_SUCCESS
}

/// Spawns a new Account.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
/// use svm_types::Address};
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
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, &mut error) };
/// assert!(res.is_ok());
///
/// let mut receipt = svm_byte_array::default();
/// let mut init_state = svm_byte_array::default();
///
/// let envelope = svm_byte_array::default();
/// let message = svm_byte_array::default();
/// let context = svm_byte_array::default();
/// let gas_enabled = false;
///
/// let _res = unsafe {
///   svm_spawn(
///     &mut receipt,
///     runtime,
///     envelope,
///     message,
///     context,
///     gas_enabled,
///     &mut error)
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_spawn(
    _receipt: *mut svm_byte_array,
    _runtime: *mut c_void,
    _envelope: svm_byte_array,
    _message: svm_byte_array,
    _context: svm_byte_array,
    _gas_enabled: bool,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_spawn` start");

    todo!("extract `spawner, gas_limit`");

    // let runtime: &mut Box<dyn Runtime> = runtime.into();

    // let gas_limit = maybe_gas(gas_enabled, gas_limit);
    // let rust_receipt = runtime.spawn(envelope, message.as_bytes(), context);
    // let receipt_bytes = receipt::encode_spawn(&rust_receipt);

    // // Returns the encoded `SpawnReceipt` as `svm_byte_array`.
    // //
    // // # Notes:
    // //
    // // Should call later `svm_receipt_destroy`
    // data_to_svm_byte_array(SPAWN_RECEIPT_TYPE, receipt, receipt_bytes);

    // debug!("`svm_spawn` returns `SVM_SUCCESS`");

    // svm_result_t::SVM_SUCCESS
}

/// Triggers a `Call Account` transaction.
/// Returns the Receipt of the execution via the `receipt` parameter.
///
/// # Examples
///
/// ```rust, no_run
/// use std::ffi::c_void;
///
/// use svm_runtime_ffi::*;
///
/// use svm_types::{State, Address,};
/// use svm_ffi::svm_byte_array;
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
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, &mut error) };
/// assert!(res.is_ok());
///
/// let mut receipt = svm_byte_array::default();
/// let envelope = svm_byte_array::default();
/// let message = svm_byte_array::default();
/// let context = svm_byte_array::default();
/// let gas_enabled = false;
///
/// let _res = unsafe {
///   svm_call(
///     &mut receipt,
///     runtime,
///     envelope,
///     message,
///     context,
///     gas_limit,
///     &mut error)
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_call(
    _receipt: *mut svm_byte_array,
    _runtime: *mut c_void,
    _envelope: svm_byte_array,
    _message: svm_byte_array,
    _context: svm_byte_array,
    _gas_enabled: bool,
    _error: *mut svm_byte_array,
) -> svm_result_t {
    debug!("`svm_call` start");

    todo!("extract `state` (from `context`), `gas_limit` (from `envelope`)");

    // let runtime: &mut Box<dyn Runtime> = runtime.into();

    // let rust_receipt = runtime.call(&envelope, message.as_bytes(), &context);
    // let receipt_bytes = receipt::encode_call(&rust_receipt);

    // Returns encoded `CallReceipt` as `svm_byte_array`.
    //
    // # Notes:
    //
    // Should call later `svm_receipt_destroy`
    // data_to_svm_byte_array(CALL_RECEIPT_TYPE, receipt, receipt_bytes);

    // debug!("`svm_call` returns `SVM_SUCCESS`");

    // svm_result_t::SVM_SUCCESS
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

/// Destroys a resource holding a type textual name
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_type_name_destroy(ptr: *mut svm_byte_array) {
    let ptr = svm_ffi::from_raw(svm_ffi::SVM_RESOURCE_NAME_PTR_TYPE, ptr);

    svm_byte_array_destroy(ptr)
}

/// Destroys the Runtime and its associated resources.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_types::Address;
/// use svm_ffi::svm_byte_array;
///
/// // create runtime
///
/// let mut state_kv = std::ptr::null_mut();
/// let res = unsafe { svm_memory_state_kv_create(&mut state_kv) };
/// assert!(res.is_ok());
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, state_kv, &mut error) };
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

/// Frees `svm_byte_array`
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
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
    let bytes = msg.to_vec();
    let err: svm_byte_array = (svm_ffi::SVM_WASM_ERROR_TYPE, bytes).into();

    let ty = svm_ffi::SVM_WASM_ERROR_TYPE_PTR;
    let err = svm_ffi::into_raw(ty, err);

    svm_ffi::as_mut(err)
}
