use std::cell::{Ref, RefCell, RefMut};
use std::ffi::c_void;
use std::rc::Rc;

use log::debug;
use wasmer::Memory;

use svm_storage::app::AppStorage;
use svm_types::{gas::MaybeGas, receipt::Log, HostCtx};

/// `Context` is a container for the accessible data by `wasmer` instances.
///
/// * `host`         - A pointer to the `Host`.
/// * `host_ctx`     - A pointer to the `HostCtx` (i.e: `sender`, `block_id`, `nonce`, ...).
/// * `storage`      - Instance's `AppStorage`.
/// * `gas_metering` - Whether gas metering is enabled.

#[derive(Clone)]
pub struct Context {
    inner: Rc<RefCell<ContextInner>>,
}

impl Context {
    pub fn new(
        host: *mut c_void,
        host_ctx: HostCtx,
        gas_limit: MaybeGas,
        storage: AppStorage,
    ) -> Self {
        let inner = ContextInner::new(host, host_ctx, gas_limit, storage);

        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn borrow(&self) -> Ref<ContextInner> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<ContextInner> {
        self.inner.borrow_mut()
    }
}

pub struct ContextInner {
    /// A pointer to the `host`.
    ///
    /// For example, `host` will point a to struct having an access to the balance of each account.
    pub host: *mut c_void,

    /// Raw pointer to host context fields.
    pub host_ctx: HostCtx,

    /// Gas limit (relevant only when `gas_metering = true`)
    pub gas_limit: u64,

    /// Whether gas metering is enabled or not
    pub gas_metering: bool,

    /// An accessor to the App's storage
    pub storage: AppStorage,

    /// App's logs
    pub logs: Vec<Log>,

    /// Instance's memory
    memory: Option<Memory>,

    /// Pointer to calldata. Tuple stores `(offset, len)`.
    calldata: Option<(usize, usize)>,
}

impl ContextInner {
    fn new(host: *mut c_void, host_ctx: HostCtx, gas_limit: MaybeGas, storage: AppStorage) -> Self {
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
            memory: None,
            calldata: None,
        }
    }

    pub fn set_calldata(&mut self, offset: usize, len: usize) {
        self.calldata = Some((offset, len));
    }

    pub fn get_calldata(&self) -> (usize, usize) {
        self.calldata.unwrap()
    }

    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }

    pub fn get_memory(&self) -> &Memory {
        debug_assert!(self.memory.is_some());

        self.memory.as_ref().unwrap()
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}
