// Let's tune "rustc" to be super annoying and that only what must be
// `unsafe` actually is.
#![deny(unsafe_op_in_unsafe_fn)]

use log::{debug, error};

use std::ffi::c_void;

use svm_codec::Codec;
use svm_runtime::Runtime;
use svm_types::{Context, Envelope, Type};

use crate::r#ref::RuntimeRef;
use crate::tracking;
use crate::{raw_error, raw_io_error, raw_validate_error, svm_result_t};
use crate::{svm_byte_array, svm_resource_iter_t, svm_resource_t};

static ENVELOPE_TYPE: Type = Type::Str("Tx Envelope");
static MESSAGE_TYPE: Type = Type::Str("Tx Message");
static CONTEXT_TYPE: Type = Type::Str("Tx Context");
static DEPLOY_RECEIPT_TYPE: Type = Type::Str("Deploy Receipt");
static SPAWN_RECEIPT_TYPE: Type = Type::Str("Spawn Receipt");
static VERIFY_RECEIPT_TYPE: Type = Type::Str("Verify Receipt");
static CALL_RECEIPT_TYPE: Type = Type::Str("Call Receipt");

pub static SVM_RESOURCE_TYPE: Type = Type::of::<svm_resource_t>();
pub static SVM_RESOURCES_ITER_TYPE: Type = Type::of::<svm_resource_iter_t>();
pub static SVM_RESOURCE_NAME_TYPE: Type = Type::Str("resource-name");
pub static SVM_RESOURCE_NAME_PTR_TYPE: Type = Type::Str("resource-name ptr");

///
/// Start of the Public C-API
///
/// * Each method is annotated with `#[no_mangle]`
/// * Each method has `unsafe extern "C"` before `fn`
///
/// See `build.rs` for using `cbindgen` to generate `svm.h`
///
///
/// Creates a new SVM Runtime instance backed-by an in-memory KV.
///
/// Returns it the created Runtime via the `runtime` parameter.
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
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
    let error = unsafe { error.as_mut().unwrap() };

    catch_unwind_or_fail(error, || {
        use svm_runtime::testing;

        let runtime = unsafe { runtime.as_uninit_mut() }
            .unwrap()
            .write(std::ptr::null_mut());

        debug!("`svm_memory_runtime_create` start");

        let mem_runtime = testing::create_memory_runtime();
        let res = into_raw_runtime(runtime, mem_runtime);

        debug!("`svm_memory_runtime_create` end");

        res
    })
}

/// Destroys the Runtime and its associated resources.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
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
    let _ = unsafe { RuntimeRef::from_raw(runtime) };
}

/// Allocates `svm_byte_array` to be used later for passing a binary [`Envelope`].
///
/// The number of allocated bytes is a fixed, and it equals to [`svm_codec::envelope::byte_size()`](svm_codec::envelope::byte_size).
#[must_use]
#[no_mangle]
pub extern "C" fn svm_envelope_alloc() -> svm_byte_array {
    let size = Envelope::fixed_size().unwrap();
    svm_byte_array::with_capacity(size, ENVELOPE_TYPE)
}

/// Allocates `svm_byte_array` of `size` bytes, meant to be used for passing a binary [`Message`].
#[must_use]
#[no_mangle]
pub extern "C" fn svm_message_alloc(size: u32) -> svm_byte_array {
    svm_byte_array::with_capacity(size as usize, MESSAGE_TYPE)
}

/// Allocates `svm_byte_array` to be used later for passing a binary [`Context`].
///
/// The number of allocated bytes is a fixed, and it equals to [`svm_codec::context::byte_size()`](svm_codec::context::byte_size).
#[must_use]
#[no_mangle]
pub extern "C" fn svm_context_alloc() -> svm_byte_array {
    let size = Context::fixed_size().unwrap();
    svm_byte_array::with_capacity(size, CONTEXT_TYPE)
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
    let error = || unsafe { error.as_mut() }.unwrap();

    catch_unwind_or_fail(error(), || {
        let runtime = runtime.cast::<Box<dyn Runtime>>();
        let runtime = unsafe { runtime.as_mut() }.unwrap();

        let message = message.as_slice();
        match runtime.validate_deploy(message) {
            Ok(()) => {
                debug!("`svm_validate_deploy` returns `SVM_SUCCESS`");
                svm_result_t::SVM_SUCCESS
            }
            Err(e) => {
                error!("`svm_validate_deploy` returns `SVM_FAILURE`");
                raw_validate_error(&e, error());
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
    let error = || unsafe { error.as_mut() }.unwrap();

    catch_unwind_or_fail(error(), || {
        let runtime = runtime.cast::<Box<dyn Runtime>>();
        let runtime = unsafe { runtime.as_mut() }.unwrap();

        match runtime.validate_spawn(message.as_slice()) {
            Ok(()) => {
                debug!("`svm_validate_spawn` returns `SVM_SUCCESS`");
                svm_result_t::SVM_SUCCESS
            }
            Err(e) => {
                error!("`svm_validate_spawn` returns `SVM_FAILURE`");
                raw_validate_error(&e, error());
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
    let error = || unsafe { error.as_mut() }.unwrap();

    catch_unwind_or_fail(error(), || {
        let runtime = runtime.cast::<Box<dyn Runtime>>();
        let runtime = unsafe { runtime.as_mut() }.unwrap();

        debug!("`svm_validate_call` start");

        match runtime.validate_call(message.as_slice()) {
            Ok(()) => {
                debug!("`svm_validate_call` returns `SVM_SUCCESS`");
                svm_result_t::SVM_SUCCESS
            }
            Err(e) => {
                error!("`svm_validate_call` returns `SVM_FAILURE`");
                raw_validate_error(&e, error());
                svm_result_t::SVM_FAILURE
            }
        }
    })
}

/// Deploys a `Template`
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
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
    let error = || unsafe { error.as_mut() }.unwrap();

    catch_unwind_or_fail(error(), || {
        let runtime = runtime.cast::<Box<dyn Runtime>>();
        let runtime = unsafe { runtime.as_mut() }.unwrap();
        let receipt = unsafe { receipt.as_mut() }.unwrap();

        debug!("`svm_deploy` start`");

        let message = message.as_slice();

        let envelope = Envelope::decode_bytes(envelope);
        if let Err(e) = envelope {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let context = Context::decode_bytes(context);
        if let Err(e) = context {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let envelope = envelope.unwrap();
        let context = context.unwrap();
        let rust_receipt = runtime.deploy(&envelope, &message, &context);
        let receipt_bytes = rust_receipt.encode_to_vec();

        // returning encoded `TemplateReceipt` as `svm_byte_array`.
        //
        // # Notes
        //
        // Should call later `svm_receipt_destroy`
        data_to_svm_byte_array(DEPLOY_RECEIPT_TYPE, receipt, receipt_bytes);

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
    let error = || unsafe { error.as_mut() }.unwrap();

    catch_unwind_or_fail(error(), || {
        let runtime = runtime.cast::<Box<dyn Runtime>>();
        let runtime = unsafe { runtime.as_mut() }.unwrap();
        let receipt = unsafe { receipt.as_mut() }.unwrap();

        debug!("`svm_spawn` start");

        let message = message.as_slice();

        let envelope = Envelope::decode_bytes(envelope);
        if let Err(e) = envelope {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let context = Context::decode_bytes(context);
        if let Err(e) = context {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let envelope = envelope.unwrap();
        let context = context.unwrap();
        let rust_receipt = runtime.spawn(&envelope, &message, &context);
        let receipt_bytes = rust_receipt.encode_to_vec();

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

/// Calls `verify` on an Account.
/// The inputs `envelope`, `message` and `context` should be the same ones
/// passed later to `svm_call`.(in case the `verify` succeeds).
///
/// Returns the Receipt of the execution via the `receipt` parameter.
///
/// # Examples
///
/// ```rust, no_run
/// use std::ffi::c_void;
///
/// use svm_runtime_ffi::*;
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
///   svm_verify(
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
pub unsafe extern "C" fn svm_verify(
    receipt: *mut svm_byte_array,
    runtime: *mut c_void,
    envelope: svm_byte_array,
    message: svm_byte_array,
    context: svm_byte_array,
    error: *mut svm_byte_array,
) -> svm_result_t {
    let error = || unsafe { error.as_mut() }.unwrap();

    catch_unwind_or_fail(error(), || {
        let runtime = runtime.cast::<Box<dyn Runtime>>();
        let runtime = unsafe { runtime.as_mut() }.unwrap();
        let receipt = unsafe { receipt.as_mut() }.unwrap();

        debug!("`svm_verify` start");

        let message = message.as_slice();

        let envelope = Envelope::decode_bytes(envelope);
        if let Err(e) = envelope {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let context = Context::decode_bytes(context);
        if let Err(e) = context {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let envelope = envelope.unwrap();
        let context = context.unwrap();
        let rust_receipt = runtime.verify(&envelope, &message, &context);
        let receipt_bytes = rust_receipt.encode_to_vec();

        // Returns encoded `CallReceipt` as `svm_byte_array`.
        //
        // # Notes:
        //
        // Should call later `svm_receipt_destroy`
        data_to_svm_byte_array(VERIFY_RECEIPT_TYPE, &mut *receipt, receipt_bytes);

        debug!("`svm_verify` returns `SVM_SUCCESS`");
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
    let error = || unsafe { error.as_mut() }.unwrap();

    catch_unwind_or_fail(error(), || {
        let runtime = runtime.cast::<Box<dyn Runtime>>();
        let runtime = unsafe { runtime.as_mut() }.unwrap();
        let receipt = unsafe { receipt.as_mut() }.unwrap();

        debug!("`svm_call` start");

        let message = message.as_slice();

        let envelope = Envelope::decode_bytes(envelope);
        if let Err(e) = envelope {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let context = Context::decode_bytes(context);
        if let Err(e) = context {
            raw_io_error(e, error());
            return svm_result_t::SVM_FAILURE;
        }

        let envelope = envelope.unwrap();
        let context = context.unwrap();
        let rust_receipt = runtime.call(&envelope, &message, &context);
        let receipt_bytes = rust_receipt.encode_to_vec();

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
pub extern "C" fn svm_total_live_resources() -> i32 {
    std::panic::catch_unwind(tracking::total_live).unwrap_or(-1)
}

/// Initializes a new iterator over the manually-managed resources
#[must_use]
#[no_mangle]
pub extern "C" fn svm_resource_iter_new() -> *mut c_void {
    let ty = SVM_RESOURCES_ITER_TYPE;
    let snapshot = tracking::take_snapshot();

    crate::into_raw(ty, snapshot)
}

/// Destroys the manually-managed resources iterator
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_destroy(iter: *mut c_void) {
    let ty = SVM_RESOURCES_ITER_TYPE;
    let iter = unsafe { iter.as_mut() }.unwrap();

    let _ = unsafe { crate::from_raw(ty, iter) };
}

/// Returns the next manually-managed resource.
/// If there is no resource to return, returns `NULL`
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_next(iter: *mut c_void) -> *mut svm_resource_t {
    let iter = iter as *mut svm_resource_iter_t;
    let iter = unsafe { iter.as_mut() }.unwrap();

    match iter.next() {
        None => std::ptr::null_mut(),
        Some(resource) => {
            let ty = SVM_RESOURCE_TYPE;
            let ptr = crate::into_raw(ty, resource);

            ptr as *mut svm_resource_t
        }
    }
}

/// Destroy the resource
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_destroy(resource: *mut svm_resource_t) {
    let _ = unsafe { crate::from_raw(SVM_RESOURCE_TYPE, resource) };
}

/// Given a type in an interned form, returns its textual name
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_type_name_resolve(ty: usize) -> *mut svm_byte_array {
    match tracking::interned_type_rev(ty) {
        Some(ty) => {
            let ty = format!("{}", ty);
            let ty: svm_byte_array = (SVM_RESOURCE_NAME_TYPE, ty).into();

            let ptr = crate::into_raw(SVM_RESOURCE_NAME_PTR_TYPE, ty);
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
        let ptr = unsafe { crate::from_raw(SVM_RESOURCE_NAME_PTR_TYPE, ptr) };

        unsafe { svm_byte_array_destroy(ptr) }
    })
    .ok();
}

/// Frees `svm_byte_array`
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let bytes = svm_byte_array::default();
/// unsafe { svm_byte_array_destroy(bytes); }
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_byte_array_destroy(bytes: svm_byte_array) {
    unsafe { bytes.destroy() }
}

fn catch_unwind_or_fail<F>(error: &mut svm_byte_array, f: F) -> svm_result_t
where
    F: FnOnce() -> svm_result_t + std::panic::UnwindSafe,
{
    std::panic::catch_unwind(f).unwrap_or_else(|_| {
        raw_error("Internal SVM failure. This is a bug and we'd appreciate a bug report. Please provide any information that was printed to stderr.".to_string(), error);
        svm_result_t::SVM_FAILURE
    })
}

fn data_to_svm_byte_array(ty: Type, byte_array: &mut svm_byte_array, data: Vec<u8>) {
    *byte_array = svm_byte_array::from((ty, data));
}

fn into_raw_runtime<R: Runtime + 'static>(
    raw_runtime: &mut *mut c_void,
    runtime: R,
) -> svm_result_t {
    let runtime_ptr = RuntimeRef::new(Box::new(runtime));

    // # Notes
    //
    // `svm_runtime_destroy` should be called later for freeing memory.
    *raw_runtime = RuntimeRef::into_raw(runtime_ptr);

    svm_result_t::SVM_SUCCESS
}
