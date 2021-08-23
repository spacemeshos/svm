use std::ffi::c_void;

use svm_types::Type;

use crate::r#ref::RuntimeRef;
use crate::{
    api_safer, raw_error, svm_byte_array, svm_resource_iter_t, svm_resource_t, svm_result_t,
};

static SVM_RESOURCES_ITER_TYPE: Type = Type::of::<svm_resource_iter_t>();

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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_memory_runtime_create(&mut *runtime)
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
    let _ = RuntimeRef::from_raw(runtime);
}

/// Allocates `svm_byte_array` to be used later for passing a binary [`Envelope`].
///
/// The number of allocated bytes is a fixed, and it equals to [`svm_codec::envelope::byte_size()`](svm_codec::envelope::byte_size).
#[must_use]
#[no_mangle]
pub extern "C" fn svm_envelope_alloc() -> svm_byte_array {
    api_safer::svm_envelope_alloc()
}

/// Allocates `svm_byte_array` of `size` bytes, meant to be used for passing a binary [`Message`].
#[must_use]
#[no_mangle]
pub extern "C" fn svm_message_alloc(size: u32) -> svm_byte_array {
    api_safer::svm_message_alloc(size)
}

/// Allocates `svm_byte_array` to be used later for passing a binary [`Context`].
///
/// The number of allocated bytes is a fixed, and it equals to [`svm_codec::context::byte_size()`](svm_codec::context::byte_size).
#[must_use]
#[no_mangle]
pub extern "C" fn svm_context_alloc() -> svm_byte_array {
    api_safer::svm_context_alloc()
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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_validate_deploy(&mut *(runtime as *mut _), message, &mut *error)
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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_validate_spawn(RuntimeRef::as_native(runtime), message, &mut *error)
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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_validate_call(RuntimeRef::as_native(runtime), message, &mut *error)
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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_deploy(
            &mut *receipt,
            &mut *(runtime as *mut _),
            envelope,
            message,
            context,
            &mut *error,
        )
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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_spawn(
            &mut *receipt,
            &mut *(runtime as *mut _),
            envelope,
            message,
            context,
            &mut *error,
        )
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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_verify(
            &mut *receipt,
            &mut *(runtime as *mut _),
            envelope,
            message,
            context,
            &mut *error,
        )
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
    catch_unwind_or_fail(&mut *error, || {
        api_safer::svm_call(
            &mut *receipt,
            &mut *(runtime as *mut _),
            envelope,
            message,
            context,
            &mut *error,
        )
    })
}

/// Returns the total live manually-managed resources.
#[must_use]
#[no_mangle]
pub extern "C" fn svm_total_live_resources() -> i32 {
    api_safer::svm_total_live_resources()
}

/// Initializes a new iterator over the manually-managed resources
#[must_use]
#[no_mangle]
pub extern "C" fn svm_resource_iter_new() -> *mut c_void {
    api_safer::svm_resource_iter_new()
}

/// Destroys the manually-managed resources iterator
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_destroy(iter: *mut c_void) {
    let ty = SVM_RESOURCES_ITER_TYPE;
    let _ = crate::from_raw(ty, &mut *iter);
}

/// Returns the next manually-managed resource.
/// If there is no resource to return, returns `NULL`
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_iter_next(iter: *mut c_void) -> *mut svm_resource_t {
    let iter = iter as *mut svm_resource_iter_t;
    api_safer::svm_resource_iter_next(&mut *iter)
}

/// Destroy the resource
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_destroy(resource: *mut svm_resource_t) {
    let _ = crate::from_raw(api_safer::SVM_RESOURCE_TYPE, resource);
}

/// Given a type in an interned form, returns its textual name
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_type_name_resolve(ty: usize) -> *mut svm_byte_array {
    api_safer::svm_resource_type_name_resolve(ty)
}

/// Destroys a resource holding a type textual name
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_resource_type_name_destroy(ptr: *mut svm_byte_array) {
    let _ = crate::from_raw(api_safer::SVM_RESOURCE_NAME_PTR_TYPE, ptr);
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
    bytes.destroy()
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
