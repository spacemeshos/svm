use log::{debug, error};

use std::{ffi::c_void, panic::UnwindSafe};

#[cfg(feature = "default-rocksdb")]
use std::path::Path;

use svm_codec::receipt;
use svm_ffi::{svm_byte_array, svm_resource_iter_t, svm_resource_t, tracking};
use svm_runtime::{Runtime, RuntimePtr};
use svm_types::{Context, Envelope, Type};

#[cfg(feature = "default-rocksdb")]
use crate::raw_utf8_error;

use crate::{error::raw_error, raw_io_error, raw_validate_error, svm_result_t};

static ENVELOPE_TYPE: Type = Type::Str("Tx Envelope");
static MESSAGE_TYPE: Type = Type::Str("Tx Message");
static CONTEXT_TYPE: Type = Type::Str("Tx Context");
static DEPLOY_RECEIPT_TYPE: Type = Type::Str("Deploy Receipt");
static SPAWN_RECEIPT_TYPE: Type = Type::Str("Spawn Receipt");
static CALL_RECEIPT_TYPE: Type = Type::Str("Call Receipt");

fn catch_unwind_with_err<T, F>(error: &mut svm_byte_array, default: T, f: F) -> T
where
    F: FnOnce() -> T + UnwindSafe,
{
    std::panic::catch_unwind(f).unwrap_or_else(|_| {
        raw_error("Internal SVM failure. This is a bug and we'd appreciate a bug report. Please provide any information that was printed to stderr.".to_string(), error);
        default
    })
}

#[inline]
fn data_to_svm_byte_array(ty: Type, byte_array: &mut svm_byte_array, data: Vec<u8>) {
    let (ptr, len, cap) = data.into_raw_parts();

    tracking::increment_live(ty);

    let length = len as u32;
    let capacity = cap as u32;
    let type_id = tracking::interned_type(ty);

    *byte_array = unsafe { svm_byte_array::from_raw_parts(ptr, length, capacity, type_id) };
}

unsafe fn into_raw_runtime<R: Runtime + 'static>(
    raw_runtime: *mut *mut c_void,
    runtime: R,
) -> svm_result_t {
    let runtime_ptr = RuntimePtr::new(Box::new(runtime));

    // # Notes
    //
    // `svm_runtime_destroy` should be called later for freeing memory.
    *raw_runtime = RuntimePtr::into_raw(runtime_ptr);

    svm_result_t::SVM_SUCCESS
}

#[must_use]
unsafe fn decode_envelope(envelope: svm_byte_array) -> std::io::Result<Envelope> {
    use std::io::Cursor;
    use svm_codec::envelope;

    let mut cursor = Cursor::new(envelope.as_slice());
    envelope::decode(&mut cursor)
}

#[must_use]
unsafe fn decode_context(context: svm_byte_array) -> std::io::Result<Context> {
    use std::io::Cursor;
    use svm_codec::context;

    let mut cursor = Cursor::new(context.as_slice());
    context::decode(&mut cursor)
}

/// Allocates `svm_byte_array` of `size` bytes, destined to be used for passing a binary [`Envelope`].
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_envelope_alloc(size: u32) -> svm_byte_array {
    svm_byte_array::with_capacity(size as usize, ENVELOPE_TYPE)
}

/// Allocates `svm_byte_array` of `size` bytes, destined to be used for passing a binary [`Message`].
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_message_alloc(size: u32) -> svm_byte_array {
    svm_byte_array::with_capacity(size as usize, MESSAGE_TYPE)
}

/// Allocates `svm_byte_array` of `size` bytes, destined to be used for passing a binary [`Context`].
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_context_alloc(size: u32) -> svm_byte_array {
    svm_byte_array::with_capacity(size as usize, CONTEXT_TYPE)
}

/// Validates syntactically a binary `Deploy Template` transaction.
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
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
///
/// let message = svm_byte_array::default();
///
/// let _res = unsafe { svm_validate_deploy(runtime, message, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_deploy(
    runtime: *mut c_void,
    message: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind_with_err(&mut *error, svm_result_t::SVM_FAILURE, || {
        let runtime: &mut Box<dyn Runtime> = runtime.into();

        match runtime.validate_deploy(message.as_slice()) {
            Ok(()) => {
                debug!("`svm_validate_deploy` returns `SVM_SUCCESS`");
                svm_result_t::SVM_SUCCESS
            }
            Err(e) => {
                error!("`svm_validate_deploy` returns `SVM_FAILURE`");
                raw_validate_error(&e, &mut *error);
                svm_result_t::SVM_FAILURE
            }
        }
    })
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
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
///
/// let message = svm_byte_array::default();
/// let _res = unsafe { svm_validate_spawn(runtime, message, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_spawn(
    runtime: *mut c_void,
    message: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind_with_err(&mut *error, svm_result_t::SVM_FAILURE, || {
        let runtime: &mut Box<dyn Runtime> = runtime.into();
        let message = message.as_slice();

        match runtime.validate_spawn(message) {
            Ok(()) => {
                debug!("`svm_validate_spawn` returns `SVM_SUCCESS`");
                svm_result_t::SVM_SUCCESS
            }
            Err(e) => {
                error!("`svm_validate_spawn` returns `SVM_FAILURE`");
                raw_validate_error(&e, &mut *error);
                svm_result_t::SVM_FAILURE
            }
        }
    })
}

/// Validates syntactically a binary `Call Account` transaction.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
///
/// let message = svm_byte_array::default();
/// let _res = unsafe { svm_validate_call(runtime, message, &mut error) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_call(
    runtime: *mut c_void,
    message: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind_with_err(&mut *error, svm_result_t::SVM_FAILURE, || {
        debug!("`svm_validate_call` start");

        let runtime: &mut Box<dyn Runtime> = runtime.into();
        let message = message.as_slice();

        match runtime.validate_call(message) {
            Ok(()) => {
                debug!("`svm_validate_call` returns `SVM_SUCCESS`");
                svm_result_t::SVM_SUCCESS
            }
            Err(e) => {
                error!("`svm_validate_call` returns `SVM_FAILURE`");
                raw_validate_error(&e, &mut *error);
                svm_result_t::SVM_FAILURE
            }
        }
    })
}

/// Creates a new SVM Runtime instance backed-by an in-memory KV.
///
/// Returns it the created Runtime via the `runtime` parameter.
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-memory")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(
    runtime: *mut *mut c_void,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind_with_err(&mut *error, svm_result_t::SVM_FAILURE, || {
        use svm_runtime::testing;

        debug!("`svm_memory_runtime_create` start");

        let mem_runtime = testing::create_memory_runtime();
        let res = into_raw_runtime(runtime, mem_runtime);

        debug!("`svm_memory_runtime_create` end");

        res
    })
}

/// Creates a new SVM Runtime instance.
/// Returns it via the `runtime` parameter.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let ty = Type::Str("path");
/// let kv_path = String::from("path for SVM internal db goes here");

/// let kv_path: svm_byte_array = (ty, kv_path).into();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_runtime_create(&mut runtime, kv_path, &mut error) };
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-rocksdb")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    runtime: *mut *mut c_void,
    kv_path: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind(&mut *error, svm_result_t::SVM_FAILURE, || {
        debug!("`svm_runtime_create` start");

        let kv_path: Result<String, std::string::FromUtf8Error> = String::try_from(kv_path);

        if kv_path.is_err() {
            raw_utf8_error(kv_path, error);
            return svm_result_t::SVM_FAILURE;
        }

        let kv_path = kv_path.unwrap();

        let rocksdb_runtime = svm_runtime::create_rocksdb_runtime(&Path::new(&kv_path));
        let res = into_raw_runtime(runtime, rocksdb_runtime);

        debug!("`svm_runtime_create` end");

        res
    })
}

/// Deploys a `Template`
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
///
/// let mut receipt = svm_byte_array::default();
/// let envelope = svm_byte_array::default();
/// let message = svm_byte_array::default();
/// let context = svm_byte_array::default();
///
/// let res = unsafe {
///   svm_deploy(
///     &mut receipt,
///     runtime,
///     envelope,
///     message,
///     context,
///     &mut error)
/// };
///
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_deploy(
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind_with_err(&mut *error, svm_result_t::SVM_FAILURE, || {
        debug!("`svm_deploy` start`");

        let runtime: &mut Box<dyn Runtime> = runtime.into();
        let message = message.as_slice();

        let envelope = decode_envelope(envelope);
        if let Err(e) = envelope {
            raw_io_error(e, &mut *error);
            return svm_result_t::SVM_FAILURE;
        }

        let context = decode_context(context);
        if let Err(e) = context {
            raw_io_error(e, &mut *error);
            return svm_result_t::SVM_FAILURE;
        }

        let envelope = envelope.unwrap();
        let context = context.unwrap();
        let rust_receipt = runtime.deploy(&envelope, &message, &context);
        let receipt_bytes = receipt::encode_deploy(&rust_receipt);

        // returning encoded `TemplateReceipt` as `svm_byte_array`.
        //
        // # Notes
        //
        // Should call later `svm_receipt_destroy`
        data_to_svm_byte_array(DEPLOY_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

        debug!("`svm_deploy` returns `SVM_SUCCESS`");
        svm_result_t::SVM_SUCCESS
    })
}

/// Spawns a new `Account`.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
///
/// let mut receipt = svm_byte_array::default();
/// let mut init_state = svm_byte_array::default();
///
/// let envelope = svm_byte_array::default();
/// let message = svm_byte_array::default();
/// let context = svm_byte_array::default();
///
/// let _res = unsafe {
///   svm_spawn(
///     &mut receipt,
///     runtime,
///     envelope,
///     message,
///     context,
///     &mut error)
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_spawn(
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind_with_err(&mut *error, svm_result_t::SVM_FAILURE, || {
        debug!("`svm_spawn` start");

        let runtime: &mut Box<dyn Runtime> = runtime.into();
        let message = message.as_slice();

        let envelope = decode_envelope(envelope);
        if let Err(e) = envelope {
            raw_io_error(e, &mut *error);
            return svm_result_t::SVM_FAILURE;
        }

        let context = decode_context(context);
        if let Err(e) = context {
            raw_io_error(e, &mut *error);
            return svm_result_t::SVM_FAILURE;
        }

        let envelope = envelope.unwrap();
        let context = context.unwrap();
        let rust_receipt = runtime.spawn(&envelope, &message, &context);
        let receipt_bytes = receipt::encode_spawn(&rust_receipt);

        // Returns the encoded `SpawnReceipt` as `svm_byte_array`.
        //
        // # Notes:
        //
        // Should call later `svm_receipt_destroy`
        data_to_svm_byte_array(SPAWN_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

        debug!("`svm_spawn` returns `SVM_SUCCESS`");

        svm_result_t::SVM_SUCCESS
    })
}

/// `Call Account` transaction.
/// Returns the Receipt of the execution via the `receipt` parameter.
///
/// # Examples
///
/// ```rust, no_run
/// use std::ffi::c_void;
///
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
///
/// let mut receipt = svm_byte_array::default();
/// let envelope = svm_byte_array::default();
/// let message = svm_byte_array::default();
/// let context = svm_byte_array::default();
///
/// let _res = unsafe {
///   svm_call(
///     &mut receipt,
///     runtime,
///     envelope,
///     message,
///     context,
///     &mut error)
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_call(
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    catch_unwind_with_err(&mut *error, svm_result_t::SVM_FAILURE, || {
        debug!("`svm_call` start");

        let runtime: &mut Box<dyn Runtime> = runtime.into();
        let message = message.as_slice();

        let envelope = decode_envelope(envelope);
        if let Err(e) = envelope {
            raw_io_error(e, &mut *error);
            return svm_result_t::SVM_FAILURE;
        }

        let context = decode_context(context);
        if let Err(e) = context {
            raw_io_error(e, &mut *error);
            return svm_result_t::SVM_FAILURE;
        }

        let envelope = envelope.unwrap();
        let context = context.unwrap();
        let rust_receipt = runtime.call(&envelope, &message, &context);
        let receipt_bytes = receipt::encode_call(&rust_receipt);

        // Returns encoded `CallReceipt` as `svm_byte_array`.
        //
        // # Notes:
        //
        // Should call later `svm_receipt_destroy`
        data_to_svm_byte_array(CALL_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

        debug!("`svm_call` returns `SVM_SUCCESS`");
        svm_result_t::SVM_SUCCESS
    })
}

/// Returns the total live manually-managed resources.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_total_live_resources() -> i32 {
    std::panic::catch_unwind(tracking::total_live).unwrap_or(-1)
}

/// Initializes a new iterator over the manually-managed resources
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_new() -> *mut c_void {
    std::panic::catch_unwind(|| {
        let ty = svm_ffi::SVM_RESOURCES_ITER_TYPE;
        let snapshot = tracking::take_snapshot();

        svm_ffi::into_raw(ty, snapshot)
    })
    .unwrap_or(std::ptr::null_mut())
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
    std::panic::catch_unwind(|| {
        let ptr = svm_ffi::from_raw(svm_ffi::SVM_RESOURCE_NAME_PTR_TYPE, ptr);

        svm_byte_array_destroy(ptr)
    })
    .unwrap_or(())
}

/// Destroys the Runtime and its associated resources.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// use svm_ffi::svm_byte_array;
///
/// let mut runtime = std::ptr::null_mut();
/// let mut error = svm_byte_array::default();
/// let res = unsafe { svm_memory_runtime_create(&mut runtime, &mut error) };
/// assert!(res.is_ok());
///
/// // Destroys the Runtime
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
