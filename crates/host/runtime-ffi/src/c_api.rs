/// SVM C-API methods to access the runtime.
use lazy_static::lazy_static;
use log::{debug, error};
use std::sync::Mutex;
use tokio::runtime::Runtime as TokioRuntime;

use std::ffi::c_void;
use std::panic::UnwindSafe;
use std::slice;
use std::sync::Arc;

use svm_codec::Codec;
use svm_genesis_config::GenesisConfig;
use svm_runtime::{PriceResolverRegistry, Runtime, TemplatePriceCache, ValidateError};
use svm_state::GlobalState;
use svm_types::{Address, BytesPrimitive, Context, Envelope, Layer, State};

use crate::resource_tracker::ResourceTracker;
use crate::svm_result_t;

lazy_static! {
    static ref RUNTIME_TRACKER: ResourceTracker<Runtime> = ResourceTracker::default();
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

/// Initializes the SVM library.
#[must_use]
#[no_mangle]
pub extern "C" fn svm_init() -> svm_result_t {
    *INITIALIZED.lock().unwrap() = true;
    svm_result_t::OK
}

/// Frees the memory allocated within the given [`svm_result_t`].
#[no_mangle]
pub unsafe extern "C" fn svm_free_result(_result: svm_result_t) {}

/// Start of the public C API.
/// See `build.rs` for using `cbindgen` to generate `svm.h`
///
/// Creates a new SVM runtime instance. The database for this runtime will be
/// located by `path` and `path_len`. In case `path` is `NULL`, the runtime will
/// not be persisted and will live entirely in-memory.
///
/// The pointer to the runtime is written to `runtime_ptr`.
///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::*;
///
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
///
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
/// assert!(res.is_ok());
/// ```
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    runtime_ptr: *mut *mut c_void,
    path: *const u8,
    path_len: u32,
) -> svm_result_t {
    catch_unwind_or_fail(|| {
        let mut initialized = INITIALIZED.lock().unwrap();
        if !*initialized {
            return svm_result_t::new_error(b"`svm_init` not called beforehand.");
        }
        *initialized = true;

        let genesis = GenesisConfig::mainnet();
        let tokio_rt = TokioRuntime::new().unwrap();
        let global_state = if path.is_null() {
            tokio_rt.block_on(GlobalState::in_memory(genesis))
        } else {
            let db_path_bytes = std::slice::from_raw_parts(path, path_len as usize);
            let db_path = std::str::from_utf8(db_path_bytes).expect("Invalid UTF-8 path.");
            tokio_rt.block_on(GlobalState::new(db_path, genesis))
        };

        let registry = PriceResolverRegistry::default();
        let runtime = Runtime::new(
            Arc::new(tokio_rt),
            global_state,
            TemplatePriceCache::new(registry),
        );

        *runtime_ptr = RUNTIME_TRACKER.alloc(runtime);
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
///
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
pub extern "C" fn svm_runtimes_count() -> u64 {
    RUNTIME_TRACKER.count()
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
///
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
///
/// svm_init().unwrap();
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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
/// svm_init().unwrap();
///
/// let mut runtime = std::ptr::null_mut();
/// let res = unsafe { svm_runtime_create(&mut runtime, std::ptr::null(), 0) };
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

/// Contains data related to an SVM account.
///
/// See [`svm_get_account`] for more information.
#[allow(missing_docs)]
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct svm_account {
    pub address: [u8; 20],
    pub balance: u64,
    pub counter_upper_bits: u64,
    pub counter_lower_bits: u64,
    pub template_addr: [u8; 20],
}

impl svm_account {
    /// Returns the counter value of `self` as a [`u128`].
    pub fn counter(&self) -> u128 {
        (self.counter_upper_bits as u128) << 64 | self.counter_lower_bits as u128
    }
}

/// Fetches an account's balance, template address, and nonce counter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_get_account(
    runtime_ptr: *mut c_void,
    account_addr: *const u8,
    account: *mut svm_account,
) -> svm_result_t {
    assert!(!account.is_null());
    catch_unwind_or_fail(|| {
        let runtime = get_runtime(runtime_ptr);
        let account_addr = Address::new(std::slice::from_raw_parts(account_addr, Address::N));
        let account_data = runtime.get_account(&account_addr).unwrap();

        (*account).balance = account_data.0;
        (*account).counter_upper_bits = (account_data.1 >> 64) as u64;
        (*account).counter_lower_bits = account_data.1 as u64;
        (*account)
            .template_addr
            .clone_from_slice(account_data.2.as_slice());

        svm_result_t::OK
    })
}

/// Creates an account at genesis with a given balance and nonce counter.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn svm_create_genesis_account(
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
        runtime.create_genesis_account(&account_addr, "".to_string(), balance, counter)?;

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
                error!("`{}` returns an error", f_name);
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
