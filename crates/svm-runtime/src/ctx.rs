use std::ffi::c_void;

use log::debug;

use crate::helpers::DataWrapper;

use svm_storage::app::AppStorage;
use svm_types::{gas::MaybeGas, receipt::Log, HostCtx};

/// `SvmCtx` is a container for the accessible data by `wasmer` instances.
/// * `host`         - A pointer to the `Host`.
/// * `host_ctx`     - A pointer to the `HostCtx` (i.e: `sender`, `block_id`, `nonce`, ...).
/// * `storage`      - Instance's `AppStorage`.
/// * `gas_metering` - Whether gas metering is enabled.
#[repr(C)]
pub struct SvmCtx {
    /// A pointer to the `host`.
    ///
    /// For example, `host` will point a to struct having an access to the balance of each account.
    pub host: *mut c_void,

    /// Raw pointer to host context fields.
    pub host_ctx: *const HostCtx,

    /// Gas limit (relevant only when `gas_metering = true`)
    pub gas_limit: u64,

    /// Whether gas metering is enabled or not
    pub gas_metering: bool,

    /// An accessor to the app's new storage
    pub storage: AppStorage,

    pub logs: Vec<Log>,

    /// Pointer to calldata. Tuple stores `(ptr, len)`.
    pub calldata: Option<(i32, i32)>,
}

unsafe impl Sync for SvmCtx {}
unsafe impl Send for SvmCtx {}

impl SvmCtx {
    /// Initializes a new empty `SvmCtx`
    ///
    /// * `storage` - a mutably borrowed `AppStorage`
    pub fn new(
        host: DataWrapper<*mut c_void>,
        host_ctx: DataWrapper<*const c_void>,
        gas_limit: MaybeGas,
        storage: AppStorage,
    ) -> Self {
        let host = host.unwrap();
        let host_ctx = host_ctx.unwrap() as *const HostCtx;

        let gas_metering = gas_limit.is_some();
        let gas_limit = gas_limit.unwrap_or(0);
        let logs = Vec::new();

        Self {
            host,
            host_ctx,
            storage,
            gas_metering,
            gas_limit,
            logs,
            calldata: None,
        }
    }

    pub fn set_calldata(&mut self, ptr: i32, len: i32) {
        self.calldata = Some((ptr, len));
    }

    pub fn get_calldata(&self) -> (i32, i32) {
        self.calldata.unwrap()
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}

impl Drop for SvmCtx {
    fn drop(&mut self) {
        debug!("Dropping `SvmCtx`...");

        unsafe {
            let _ = Box::from_raw(self.host_ctx as *mut HostCtx);
        }
    }
}
