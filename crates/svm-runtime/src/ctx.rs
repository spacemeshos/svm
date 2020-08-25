use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::Arc;

use log::debug;

use crate::helpers::DataWrapper;

use svm_storage::app::AppStorage;
use svm_types::{gas::MaybeGas, receipt::Log, HostCtx};

/// `SvmCtx` is a container for the accessible data by `wasmer` instances.
/// * `host`         - A pointer to the `Host`.
/// * `host_ctx`     - A pointer to the `HostCtx` (i.e: `sender`, `block_id`, `nonce`, ...).
/// * `storage`      - Instance's `AppStorage`.
/// * `gas_metering` - Whether gas metering is enabled.

pub struct SvmCtx {
    inner: Arc<RefCell<SvmCtxInner>>,
}

impl SvmCtx {
    pub fn new(
        host: DataWrapper<*mut c_void>,
        host_ctx: DataWrapper<*const c_void>,
        gas_limit: MaybeGas,
        storage: AppStorage,
    ) -> Self {
        let inner = SvmCtxInner::new(host, host_ctx, gas_limit, storage);

        Self {
            inner: Arc::new(RefCell::new(inner)),
        }
    }

    pub fn host_ctx(&self) -> &HostCtx {
        let ptr: *const HostCtx = self.inner.borrow().host_ctx;

        unsafe { &*ptr }
    }

    pub fn storage(&self) -> &AppStorage {
        &self.inner.borrow().storage
    }

    pub fn storage_mut(&self) -> &mut AppStorage {
        &mut self.inner.borrow_mut().storage
    }

    pub fn set_calldata(&self, offset: usize, len: usize) {
        self.inner.borrow_mut().set_calldata(offset, len);
    }

    pub fn get_calldata(&self) -> (usize, usize) {
        return self.inner.borrow_mut().get_calldata();
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        return self.inner.borrow_mut().take_logs();
    }
}

impl Clone for SvmCtx {
    fn clone(&self) -> Self {
        SvmCtx {
            inner: self.inner.clone(),
        }
    }
}

struct SvmCtxInner {
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

    /// Pointer to calldata. Tuple stores `(offset, len)`.
    pub calldata: Option<(usize, usize)>,
}

unsafe impl Sync for SvmCtx {}
unsafe impl Send for SvmCtx {}

impl SvmCtxInner {
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

    pub fn set_calldata(&mut self, offset: usize, len: usize) {
        self.calldata = Some((offset, len));
    }

    pub fn get_calldata(&self) -> (usize, usize) {
        self.calldata.unwrap()
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}

impl Drop for SvmCtxInner {
    fn drop(&mut self) {
        debug!("Dropping `SvmCtx`...");

        unsafe {
            let _ = Box::from_raw(self.host_ctx as *mut HostCtx);
        }
    }
}
