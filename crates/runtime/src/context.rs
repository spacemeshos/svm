use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use wasmer::Memory;

use svm_storage::app::AppStorage;
use svm_types::receipt::Log;
use svm_types::{AppAddr, Gas, TemplateAddr};

/// `Context` is a container for the accessible data by `wasmer` instances.
///
/// * `storage`      - Instance's `AppStorage`.
/// * `gas_metering` - Whether gas metering is enabled.

#[derive(wasmer::WasmerEnv, Clone)]
pub struct Context {
    inner: Rc<RefCell<ContextInner>>,

    template_addr: TemplateAddr,

    app_addr: AppAddr,
}

// SVM is single-threaded.
// `Send`, `Sync` and `Clone` are required by `wasmer::WasmerEnv`
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    /// Creates a new instance
    pub fn new(
        gas_limit: Gas,
        storage: AppStorage,
        template_addr: &TemplateAddr,
        app_addr: &AppAddr,
    ) -> Self {
        let inner = ContextInner::new(gas_limit, storage);

        Self {
            inner: Rc::new(RefCell::new(inner)),
            template_addr: template_addr.clone(),
            app_addr: app_addr.clone(),
        }
    }

    /// New instance with explicit memory
    pub fn new_with_memory(
        memory: Memory,
        gas_limit: Gas,
        storage: AppStorage,
        template_addr: &TemplateAddr,
        app_addr: &AppAddr,
    ) -> Self {
        let ctx = Self::new(gas_limit, storage, template_addr, app_addr);

        ctx.borrow_mut().set_memory(memory);

        ctx
    }

    /// Returns the `Address` of the `Template` associated
    /// with the current executed `App`.
    pub fn template_addr(&self) -> &TemplateAddr {
        &self.template_addr
    }

    /// Returns the `Address` of the current executed `App`.
    pub fn app_addr(&self) -> &AppAddr {
        &self.app_addr
    }

    /// Borrows the `Context`
    #[inline]
    pub fn borrow(&self) -> Ref<ContextInner> {
        self.inner.borrow()
    }

    /// Mutably-borrows the `Context`
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<ContextInner> {
        self.inner.borrow_mut()
    }
}

pub struct ContextInner {
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
    fn new(gas_limit: Gas, storage: AppStorage) -> Self {
        let gas_metering = gas_limit.is_some();
        let gas_limit = gas_limit.unwrap_or(0);
        let logs = Vec::new();

        Self {
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
