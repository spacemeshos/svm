/// SVM C-API methods to access the runtime.
use lazy_static::lazy_static;
use log::{debug, error};
use svm_state::GlobalState;

use std::ffi::c_void;
use std::panic::UnwindSafe;
use std::path::PathBuf;
use std::slice;

use svm_codec::Codec;
use svm_runtime::{PriceResolverRegistry, Runtime, ValidateError};
use svm_types::{Address, BytesPrimitive, Context, Envelope, Layer, State, TemplateAddr};

use crate::config::Config;
use crate::resource_tracker::ResourceTracker;
use crate::svm_result_t;

lazy_static! {
    static ref RUNTIME_TRACKER: ResourceTracker<Runtime> = ResourceTracker::default();
}

fn new_runtime() -> Runtime {
    let config = Config::get();
    let imports = ("sm".to_string(), wasmer::Exports::new());
    let global_state = if let Some(db_path) = config.db_path {
        GlobalState::new(db_path.as_os_str().to_str().unwrap())
    } else {
        GlobalState::in_memory()
    };

    Runtime::new(
        imports,
        global_state,
        PriceResolverRegistry::default(),
        None,
    )
}

/// Initializes the configuration options for all newly allocates SVM runtimes.
#[must_use]
#[no_mangle]
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

/// Frees the memory allocated within the given [`svm_result_t`].
#[no_mangle]
pub unsafe extern "C" fn svm_free_result(_result: svm_result_t) {}

/// Creates an account at genesis with a given balance and nonce counter.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn svm_create_account(
    runtime_ptr: *mut c_void,
    addr: *const u8,
    balance: u64,
    counter_upper_bits: u64,
    counter_lower_bits: u64,
) -> svm_result_t {
    catch_unwind_or_fail(|| {
        let runtime = RUNTIME_TRACKER.get(runtime_ptr).unwrap();
        let account_addr = Address::new(slice::from_raw_parts(addr, Address::N));
        let counter = ((counter_upper_bits as u128) << 64) | (counter_lower_bits as u128);
        runtime.create_account(&account_addr, "".to_string(), balance, counter)?;

        svm_result_t::OK
    })
}

/// Magically increases an account's balance by the given amount. Used for genesis setup.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn svm_increase_balance(
    runtime_ptr: *mut c_void,
    addr: *const u8,
    additional_balance: u64,
) -> svm_result_t {
    catch_unwind_or_fail(|| {
        let runtime = RUNTIME_TRACKER.get(runtime_ptr).unwrap();
        let account_addr = Address::new(slice::from_raw_parts(addr, Address::N));
        runtime.increase_balance(&account_addr, additional_balance)?;

        svm_result_t::OK
    })
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
/// let res = unsafe { svm_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(runtime_ptr: *mut *mut c_void) -> svm_result_t {
    catch_unwind_or_fail(|| {
        if !Config::is_ready() {
            return svm_result_t::new_error(b"`svm_init` not called beforehand.");
        }

        *runtime_ptr = RUNTIME_TRACKER.alloc(new_runtime());

        debug!("`svm_runtime_create` end");

        svm_result_t::OK
    })
}

/// Destroys the Runtime and its associated resources.
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// // Destroys the Runtime
/// unsafe { svm_runtime_destroy(runtime); }
/// ```
///
#[must_use]
#[no_mangle]
pub extern "C" fn svm_runtime_destroy(runtime: *mut c_void) -> svm_result_t {
    if RUNTIME_TRACKER.free(runtime).is_some() {
        svm_result_t::OK
    } else {
        svm_result_t::new_error(b"There are no allocated runtimes left to destroy!")
    }
}

/// Returns the number of currently allocated runtimes.
#[no_mangle]
pub unsafe extern "C" fn svm_runtimes_count(count: *mut u64) {
    *count = RUNTIME_TRACKER.count();
}

/// Validates syntactically a binary `Deploy Template` transaction.
///
/// Should be called while the transaction is in the `mempool` of the Host.
/// In case the transaction isn't valid - the transaction should be discarded.
///
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
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
        |r, m| r.validate_deploy(m),
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
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
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
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
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
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
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
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
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
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
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
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
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

/// Detects if the given runtime contains any uncommitted changes in memory; if
/// it does, then it returns an error wrapped inside [`svm_result_t`].
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
///
/// let mut runtime = std::ptr::null_mut();
///
/// unsafe { svm_init(true, std::ptr::null(), 0); }
///
/// let res = unsafe { svm_runtime_create(&mut runtime) };
/// assert!(res.is_ok());
///
/// let res = unsafe { svm_uncommitted_changes(runtime) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_uncommitted_changes(runtime_ptr: *mut c_void) -> svm_result_t {
    catch_unwind_or_fail(|| {
        let runtime = get_runtime(runtime_ptr);
        if runtime.has_uncommitted_changes()? {
            svm_result_t::new_error(b"The SVM global state contains uncommitted changes.")
        } else {
            svm_result_t::OK
        }
    })
}

/// Writes the current state root hash at `hash` and the current layer at `layer`.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_layer_info(
    runtime_ptr: *mut c_void,
    hash: *mut u8,
    layer: *mut u64,
) -> svm_result_t {
    catch_unwind_or_fail(|| {
        let runtime = get_runtime(runtime_ptr);
        let (layer_id, state_hash) = runtime.current_layer();
        let slice = slice::from_raw_parts_mut(hash, State::N);
        slice.clone_from_slice(&state_hash.0);
        *layer = layer_id.0;

        svm_result_t::OK
    })
}

/// Undos all changes after the given layer.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_rewind(runtime_ptr: *mut c_void, layer_id: u64) -> svm_result_t {
    catch_unwind_or_fail(|| {
        get_runtime(runtime_ptr).rewind(Layer(layer_id))?;
        svm_result_t::OK
    })
}

/// Commits all written data to persistent storage.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_commit(runtime_ptr: *mut c_void) -> svm_result_t {
    catch_unwind_or_fail(|| {
        get_runtime(runtime_ptr).commit()?;
        svm_result_t::OK
    })
}

/// Fetches an account's balance, template address, and nonce counter.
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
    assert!(!runtime_ptr.is_null());
    assert!(!account_addr.is_null());
    catch_unwind_or_fail(|| {
        let runtime = get_runtime(runtime_ptr);
        let account_addr = Address::new(std::slice::from_raw_parts(account_addr, Address::N));
        let account_data = runtime.get_account(&account_addr).unwrap();

        if !balance.is_null() {
            *balance = account_data.0;
        }
        if !counter_upper_bits.is_null() {
            *counter_upper_bits = (account_data.1 >> 64) as u64;
        }
        if !counter_lower_bits.is_null() {
            *counter_lower_bits = account_data.1 as u64;
        }
        if !template_addr.is_null() {
            let template_addr = std::slice::from_raw_parts_mut(template_addr, TemplateAddr::N);
            template_addr.clone_from_slice(account_data.2.as_slice());
        }

        svm_result_t::OK
    })
}

/// Sends coins from the current executing account to a destination account.
///
/// # Panics
///
/// Panics when the destination account does not exist.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_transfer(
    runtime_ptr: *mut c_void,
    src_addr: *const u8,
    dst_addr: *const u8,
    amount: u64,
) -> svm_result_t {
    catch_unwind_or_fail(|| {
        let runtime = get_runtime(runtime_ptr);
        let src_account_addr = Address::new(std::slice::from_raw_parts(src_addr, Address::N));
        let dst_account_addr = Address::new(std::slice::from_raw_parts(dst_addr, Address::N));
        runtime.transfer(&src_account_addr, &dst_account_addr, amount);

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
    _f_name: &str,
) -> svm_result_t
where
    F: FnOnce(&mut Runtime, &Envelope, &[u8], &Context) -> C + UnwindSafe,
    C: Codec + UnwindSafe + std::fmt::Debug,
{
    catch_unwind_or_fail(|| {
        let runtime = get_runtime(runtime_ptr);
        let message = slice::from_raw_parts(message, message_size as usize);
        let envelope = slice::from_raw_parts(envelope, Envelope::fixed_size().unwrap());
        let context = slice::from_raw_parts(context, Context::fixed_size().unwrap());

        let envelope = Envelope::decode_bytes(envelope)?;
        let context = Context::decode_bytes(context)?;
        let receipt = f(runtime, &envelope, &message, &context);

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
    F: FnOnce(&mut Runtime, &[u8]) -> Result<(), ValidateError> + UnwindSafe,
{
    catch_unwind_or_fail(|| {
        let runtime = get_runtime(runtime_ptr);
        let message = slice::from_raw_parts(message, message_size as usize);

        match validate_f(runtime, message) {
            Ok(()) => {
                debug!("`{}` returns `svm_result_t::OK`", f_name);
                svm_result_t::OK
            }
            Err(e) => {
                error!("`{}` returns an errors", f_name);
                svm_result_t::new_error(e.to_string().as_bytes())
            }
        }
    })
}

unsafe fn get_runtime(runtime_ptr: *mut c_void) -> &'static mut Runtime {
    RUNTIME_TRACKER
        .get(runtime_ptr)
        .expect("The given runtime pointer doesn't point to a valid runtime.")
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
