use log::{debug, error};

use std::ffi::c_void;
use std::panic::UnwindSafe;
use std::path::PathBuf;
use std::slice;

use svm_codec::Codec;
use svm_runtime::{DefaultRuntime, Runtime, ValidateError};
use svm_types::{Address, BytesPrimitive, Context, Envelope, Layer, TemplateAddr};

use crate::config::Config;
use crate::runtime_tracker::RuntimeTracker;
use crate::svm_result_t;

/// Initializes the configuration options for all newly allocates SVM runtimes.
pub unsafe extern "C" fn svm_init(in_memory: bool, path: *const u8, path_len: u32) -> svm_result_t {
    Config::set(Config {
        db_path: if in_memory {
            None
        } else {
            let slice = std::slice::from_raw_parts(path, path_len as usize);
            let s = std::str::from_utf8(slice).expect("Invalid UTF-8");
            Some(PathBuf::from(s.to_string()))
        },
    });

    svm_result_t::OK
}

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
/// unsafe { svm_init(true, std::ptr::null(), 0); }
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
/// ```
///
#[cfg(feature = "default-memory")]
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_memory_runtime_create(runtime: *mut *mut c_void) -> svm_result_t {
    catch_unwind_or_fail(|| {
        if !Config::is_ready() {
            return svm_result_t::new_error(b"`svm_init` not called beforehand.");
        }

        debug!("`svm_memory_runtime_create` start");
        *runtime = RuntimeTracker::alloc();
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
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// // Destroys the Runtime
/// unsafe { svm_runtime_destroy(runtime); }
/// ```
///
#[must_use]
#[no_mangle]
pub extern "C" fn svm_runtime_destroy(runtime: *mut c_void) -> svm_result_t {
    if RuntimeTracker::free(runtime).is_some() {
        svm_result_t::OK
    } else {
        svm_result_t::new_error(b"There are no allocated runtimes left to destroy!")
    }
}

/// Returns the number of currently allocated runtimes.
#[no_mangle]
pub unsafe extern "C" fn svm_runtimes_count(count: *mut u64) {
    *count = RuntimeTracker::count();
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
/// unsafe { svm_init(true, std::ptr::null(), 0); }
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
        |r, m| Runtime::validate_deploy(r, m),
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
/// unsafe { svm_init(true, std::ptr::null(), 0); }
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
        |r, m| Runtime::validate_spawn(r, m),
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
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_memory_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let message = b"message data...";
/// let _res = unsafe { svm_validate_call(runtime, message.as_ptr(), message.len() as u32) };
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
        |r, m| Runtime::validate_call(r, m),
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
/// unsafe { svm_init(true, std::ptr::null(), 0); }
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
        |r, e, m, c| Runtime::deploy(r, e, m, c),
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
/// unsafe { svm_init(true, std::ptr::null(), 0); }
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
/// unsafe { svm_init(true, std::ptr::null(), 0); }
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
        "svm_verify",
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
/// unsafe { svm_init(true, std::ptr::null(), 0); }
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

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_rewind(runtime: *mut c_void, layer_id: u64) -> svm_result_t {
    catch_unwind_or_fail(|| {
        RuntimeTracker::get(runtime)
            .unwrap()
            .rewind(Layer(layer_id))?;
        svm_result_t::OK
    })
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_commit(runtime: *mut c_void) -> svm_result_t {
    catch_unwind_or_fail(|| {
        RuntimeTracker::get(runtime).unwrap().commit()?;
        svm_result_t::OK
    })
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_get_account(
    runtime_ptr: *mut c_void,
    account_addr: *const u8,
    balance: *mut u64,
    counter_upper_bits: *mut u64,
    counter_lower_bits: *mut u64,
    template_addr: *mut u8,
) -> svm_result_t {
    catch_unwind_or_fail(|| {
        let runtime = RuntimeTracker::get(runtime_ptr).unwrap();
        let account_addr = Address::new(std::slice::from_raw_parts(account_addr, Address::N));
        let template_addr = std::slice::from_raw_parts_mut(template_addr, TemplateAddr::N);
        let account_data = runtime.get_account(&account_addr).unwrap();

        *balance = account_data.0;
        *counter_upper_bits = (account_data.1 >> 64) as u64;
        *counter_lower_bits = account_data.1 as u64;
        template_addr.clone_from_slice(account_data.2.as_slice());

        svm_result_t::OK
    })
}

unsafe fn svm_runtime_action<F, C>(
    runtime_ptr: *mut c_void,
    envelope: *const u8,
    message: *const u8,
    message_size: u32,
    context: *const u8,
    f: F,
    f_name: &str,
) -> svm_result_t
where
    F: FnOnce(&mut DefaultRuntime, &Envelope, &[u8], &Context) -> C + UnwindSafe,
    C: Codec + UnwindSafe + std::fmt::Debug,
{
    catch_unwind_or_fail(|| {
        let runtime = RuntimeTracker::get(runtime_ptr).unwrap();
        let message = slice::from_raw_parts(message, message_size as usize);
        let envelope = slice::from_raw_parts(envelope, Envelope::fixed_size().unwrap());
        let context = slice::from_raw_parts(context, Context::fixed_size().unwrap());

        debug!("`{}` start", f_name);

        let envelope = Envelope::decode_bytes(envelope)?;
        let context = Context::decode_bytes(context)?;
        let receipt = f(runtime, &envelope, &message, &context);

        debug!("`{}` returns `SVM_SUCCESS`", f_name);
        svm_result_t::new_receipt(&receipt.encode_to_vec())
    })
}

unsafe fn svm_validate<F>(
    runtime_ptr: *mut c_void,
    message: *const u8,
    message_size: u32,
    validate_f: F,
    f_name: &str,
) -> svm_result_t
where
    F: FnOnce(&mut DefaultRuntime, &[u8]) -> Result<(), ValidateError> + UnwindSafe,
{
    catch_unwind_or_fail(|| {
        let runtime = RuntimeTracker::get(runtime_ptr).unwrap();
        let message = slice::from_raw_parts(message, message_size as usize);

        match validate_f(runtime, message) {
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
    std::panic::catch_unwind(f).unwrap_or_else(|e| {
        svm_result_t::new_error(
            format!(
                "
Internal SVM failure.
This is a bug and we'd appreciate a bug report.
Please provide any information that was printed to stderr.

Panic information: {:?}
            ",
                e
            )
            .as_bytes(),
        )
    })
}
