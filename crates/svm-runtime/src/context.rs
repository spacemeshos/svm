use std::cell::{Ref, RefCell, RefMut};
use std::ffi::c_void;
use std::rc::Rc;

use log::debug;

use crate::helpers::DataWrapper;

use svm_storage::app::AppStorage;
use svm_types::{gas::MaybeGas, receipt::Log, HostCtx};

/// `Context` is a container for the accessible data by `wasmer` instances.
///
/// * `host`         - A pointer to the `Host`.
/// * `host_ctx`     - A pointer to the `HostCtx` (i.e: `sender`, `block_id`, `nonce`, ...).
/// * `storage`      - Instance's `AppStorage`.
/// * `gas_metering` - Whether gas metering is enabled.

pub struct Context {
    inner: Rc<RefCell<Inner>>,
}

impl Context {
    pub fn new(
        host: DataWrapper<*mut c_void>,
        host_ctx: DataWrapper<*const c_void>,
        gas_limit: MaybeGas,
        storage: AppStorage,
    ) -> Self {
        let inner = Inner::new(host, host_ctx, gas_limit, storage);

        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn host_ctx(&self) -> &HostCtx {
        todo!()
        // let ptr: *const HostCtx = self.inner.borrow().host_ctx;

        // unsafe { &*ptr }
    }

    pub fn borrow(&self) -> Ref<Inner> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Inner> {
        self.inner.borrow_mut()
    }

    pub fn set_calldata(&self, offset: usize, len: usize) {
        todo!()
        // self.inner.borrow_mut().set_calldata(offset, len);
    }

    pub fn get_calldata(&self) -> (usize, usize) {
        todo!()
        // return self.inner.borrow_mut().get_calldata();
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        todo!()
        // return self.inner.borrow_mut().take_logs();
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            inner: self.inner.clone(),
        }
    }
}

pub struct Inner {
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

    /// An accessor to the App's storage
    pub storage: AppStorage,

    /// App's logs
    pub logs: Vec<Log>,

    /// Pointer to calldata. Tuple stores `(offset, len)`.
    pub calldata: Option<(usize, usize)>,
}

unsafe impl Sync for Context {}
unsafe impl Send for Context {}

impl Inner {
    fn new(
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

    fn set_calldata(&mut self, offset: usize, len: usize) {
        self.calldata = Some((offset, len));
    }

    fn get_calldata(&self) -> (usize, usize) {
        self.calldata.unwrap()
    }

    fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        debug!("Dropping `Context`...");

        unsafe {
            let _ = Box::from_raw(self.host_ctx as *mut HostCtx);
        }
    }
}
