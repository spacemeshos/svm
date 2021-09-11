use log::{debug, error};

use std::slice;
use std::{ffi::c_void, panic::UnwindSafe};

use svm_codec::Codec;
use svm_runtime::{Runtime, ValidateError};
use svm_types::{Context, Envelope};

use crate::svm_result_t;

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
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-memory")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(runtime: *mut *mut c_void) -> svm_result_t {
    use svm_runtime::testing::create_memory_runtime;

    catch_unwind_or_fail(|| {
        debug!("`svm_memory_runtime_create` start");

        let boxed: Box<dyn Runtime> = Box::new(create_memory_runtime());
        unsafe { runtime.as_uninit_mut() }
            .unwrap()
            .write(Box::leak(Box::new(boxed)) as *mut _ as *mut c_void);

        debug!("`svm_memory_runtime_create` end");

        svm_result_t::OK
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
pub unsafe extern "C" fn svm_runtime_destroy(runtime: *mut c_void) -> svm_result_t {
    let _ = Box::from_raw(runtime.cast::<Box<dyn Runtime>>());

    svm_result_t::OK
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
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let message = b"message data...";
/// let _res = unsafe { svm_validate_deploy(runtime, message.as_ptr(), message.len() as u32) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_deploy(
    runtime: *mut c_void,
    message: *const u8,
    message_size: u32,
) -> svm_result_t {
    svm_validate(
        runtime,
        message,
        message_size,
        |r, m| Runtime::validate_deploy(&mut *r, m),
        "svm_validate_deploy",
    )
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
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let message = b"message data...";
/// let _res = unsafe { svm_validate_spawn(runtime, message.as_ptr(), message.len() as u32) };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_validate_spawn(
    runtime: *mut c_void,
    message: *const u8,
    message_size: u32,
) -> svm_result_t {
    svm_validate(
        runtime,
        message,
        message_size,
        |r, m| r.validate_spawn(m),
        "svm_validate_spawn",
    )
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
    message: *const u8,
    message_size: u32,
) -> svm_result_t {
    svm_validate(
        runtime,
        message,
        message_size,
        |r, m| r.validate_call(m),
        "svm_validate_call",
    )
}

/// Deploys a `Template`
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let envelope = b"envelope data...";
/// let message = b"message data...";
/// let context = b"context data...";
///
/// let _res = unsafe {
///   svm_deploy(
///     runtime,
///     envelope.as_ptr(),
///     message.as_ptr(),
///     message.len() as u32,
///     context.as_ptr())
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_deploy(
    runtime: *mut c_void,
    envelope: *const u8,
    message: *const u8,
    message_size: u32,
    context: *const u8,
) -> svm_result_t {
    svm_runtime_action(
        runtime,
        envelope,
        message,
        message_size,
        context,
        |r, e, m, c| r.deploy(e, m, c),
        "svm_deploy",
    )
}

/// Spawns a new `Account`.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let envelope = b"envelope data...";
/// let message = b"message data...";
/// let context = b"context data...";
///
/// let _res = unsafe {
///   svm_spawn(
///     runtime,
///     envelope.as_ptr(),
///     message.as_ptr(),
///     message.len() as u32,
///     context.as_ptr())
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_spawn(
    runtime: *mut c_void,
    envelope: *const u8,
    message: *const u8,
    message_size: u32,
    context: *const u8,
) -> svm_result_t {
    svm_runtime_action(
        runtime,
        envelope,
        message,
        message_size,
        context,
        |r, e, m, c| r.spawn(e, m, c),
        "svm_spawn",
    )
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
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let envelope = b"envelope data...";
/// let message = b"message data...";
/// let context = b"context data...";
///
/// let _res = unsafe {
///   svm_verify(
///     runtime,
///     envelope.as_ptr(),
///     message.as_ptr(),
///     message.len() as u32,
///     context.as_ptr())
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_verify(
    runtime: *mut c_void,
    envelope: *const u8,
    message: *const u8,
    message_size: u32,
    context: *const u8,
) -> svm_result_t {
    svm_runtime_action(
        runtime,
        envelope,
        message,
        message_size,
        context,
        |r, e, m, c| r.verify(e, m, c),
        "svm_call",
    )
}

/// `Call Account` transaction.
/// Returns the Receipt of the execution via the `receipt` parameter.
///
/// # Examples
///
/// ```rust, no_run
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let envelope = b"envelope data...";
/// let message = b"message data...";
/// let context = b"context data...";
///
/// let _res = unsafe {
///   svm_call(
///     runtime,
///     envelope.as_ptr(),
///     message.as_ptr(),
///     message.len() as u32,
///     context.as_ptr())
/// };
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_call(
    runtime: *mut c_void,
    envelope: *const u8,
    message: *const u8,
    message_size: u32,
    context: *const u8,
) -> svm_result_t {
    svm_runtime_action(
        runtime,
        envelope,
        message,
        message_size,
        context,
        |r, e, m, c| r.call(e, m, c),
        "svm_call",
    )
}

unsafe fn svm_runtime_action<F, C>(
    runtime: *mut c_void,
    envelope: *const u8,
    message: *const u8,
    message_size: u32,
    context: *const u8,
    f: F,
    f_name: &str,
) -> svm_result_t
where
    F: FnOnce(&mut dyn Runtime, &Envelope, &[u8], &Context) -> C + UnwindSafe,
    C: Codec + UnwindSafe + std::fmt::Debug,
{
    catch_unwind_or_fail(|| {
        let message = slice::from_raw_parts(message, message_size as usize);
        let envelope = slice::from_raw_parts(envelope, Envelope::fixed_size().unwrap());
        let context = slice::from_raw_parts(context, Context::fixed_size().unwrap());
        let runtime = runtime.cast::<Box<dyn Runtime>>().as_mut().unwrap();

        debug!("`{}` start", f_name);

        let envelope = Envelope::decode_bytes(envelope)?;
        let context = Context::decode_bytes(context)?;
        let receipt = f(&mut **runtime, &envelope, &message, &context);

        debug!("`{}` returns `SVM_SUCCESS`", f_name);
        svm_result_t::new_receipt(&receipt.encode_to_vec())
    })
}

unsafe fn svm_validate<F>(
    runtime: *mut c_void,
    message: *const u8,
    message_size: u32,
    validate_f: F,
    f_name: &str,
) -> svm_result_t
where
    F: FnOnce(&mut dyn Runtime, &[u8]) -> Result<(), ValidateError> + UnwindSafe,
{
    catch_unwind_or_fail(|| {
        let runtime = runtime.cast::<Box<dyn Runtime>>().as_mut().unwrap();
        let message = slice::from_raw_parts(message, message_size as usize);

        match validate_f(&mut **runtime, message) {
            Ok(()) => {
                debug!("`{}` returns `SVM_SUCCESS`", f_name);
                svm_result_t::OK
            }
            Err(e) => {
                error!("`{}` returns `SVM_FAILURE`", f_name);
                svm_result_t::new_error(e.to_string().as_bytes())
            }
        }
    })
}

fn catch_unwind_or_fail<F>(f: F) -> svm_result_t
where
    F: FnOnce() -> svm_result_t + std::panic::UnwindSafe,
{
    std::panic::catch_unwind(f).unwrap_or_else(|_| {
        svm_result_t::new_error(
            br#"
Internal SVM failure.
This is a bug and we'd appreciate a bug report.
Please provide any information that was printed to stderr."#,
        )
    })
}
