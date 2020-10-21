use std::cell::{Ref, RefCell, RefMut};
use std::ffi::c_void;
use std::rc::Rc;

use log::debug;
use wasmer::Memory;

use svm_storage::app::AppStorage;
use svm_types::{gas::MaybeGas, receipt::Log};

/// `Context` is a container for the accessible data by `wasmer` instances.
///
/// * `host`         - A pointer to the `Host`.
/// * `storage`      - Instance's `AppStorage`.
/// * `gas_metering` - Whether gas metering is enabled.

#[derive(Clone)]
pub struct Context {
    inner: Rc<RefCell<ContextInner>>,
}

impl Context {
    pub fn new(host: *mut c_void, gas_limit: MaybeGas, storage: AppStorage) -> Self {
        let inner = ContextInner::new(host, gas_limit, storage);

        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn new_with_memory(
        memory: Memory,
        host: *mut c_void,
        gas_limit: MaybeGas,
        storage: AppStorage,
    ) -> Self {
        let ctx = Self::new(host, gas_limit, storage);

        ctx.borrow_mut().set_memory(memory);

        ctx
    }

    #[inline]
    pub fn borrow(&self) -> Ref<ContextInner> {
        self.inner.borrow()
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<ContextInner> {
        self.inner.borrow_mut()
    }
}

pub struct ContextInner {
    /// A pointer to the `host`.
    ///
    /// For example, `host` will point a to struct having an access to the balance of each account.
    pub host: *mut c_void,

    /// Gas limit (relevant only when `gas_metering = true`)
    pub gas_limit: u64,

    /// Whether gas metering is enabled or not
    pub gas_metering: bool,

    /// An accessor to the App's storage
    pub storage: AppStorage,

    /// App's logs
    pub logs: Vec<Log>,

    /// Pointer to `returndata`. Tuple stores `(offset, len)`.
    pub returndata: Option<(usize, usize)>,

    /// Instance's memory
    memory: Option<Memory>,

    /// Pointer to `calldata`. Tuple stores `(offset, len)`.
    calldata: Option<(usize, usize)>,
}

impl ContextInner {
    fn new(host: *mut c_void, gas_limit: MaybeGas, storage: AppStorage) -> Self {
        let gas_metering = gas_limit.is_some();
        let gas_limit = gas_limit.unwrap_or(0);
        let logs = Vec::new();

        Self {
            host,
            storage,
            gas_metering,
            gas_limit,
            logs,
            memory: None,
            calldata: None,
            returndata: None,
        }
    }

    pub fn set_calldata(&mut self, offset: usize, len: usize) {
        self.calldata = Some((offset, len));
    }

    pub fn get_calldata(&self) -> (usize, usize) {
        debug_assert!(self.calldata.is_some());

        self.calldata.unwrap()
    }

    pub fn set_returndata(&mut self, offset: usize, len: usize) {
        debug_assert!(self.returndata.is_none());

        self.returndata = Some((offset, len));
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
