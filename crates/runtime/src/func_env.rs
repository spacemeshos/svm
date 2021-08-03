//! Implements [`FuncEnv`]. Used for managing data of running `Transaction`s.

use wasmer::Memory;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use svm_storage::account::AccountStorage;
use svm_types::{Address, Context, Envelope, ReceiptLog, TemplateAddr};

/// [`FuncEnv`] is a container for the accessible data by running [`Wasmer instance`](wasmer::Instance).
#[derive(wasmer::WasmerEnv, Clone)]
pub struct FuncEnv {
    inner: Rc<RefCell<Inner>>,
    template_addr: TemplateAddr,
    target_addr: Address,
    envelope: Envelope,
    context: Context,
}

/// # Safety
///
/// SVM is single-threaded.
/// `Send`, `Sync` and `Clone` are required by `wasmer::WasmerEnv`.
unsafe impl Send for FuncEnv {}
unsafe impl Sync for FuncEnv {}

impl FuncEnv {
    /// Creates a new instance
    pub fn new(
        storage: AccountStorage,
        envelope: &Envelope,
        context: &Context,
        template_addr: TemplateAddr,
        target_addr: Address,
    ) -> Self {
        let inner = Inner::new(storage);

        Self {
            inner: Rc::new(RefCell::new(inner)),
            template_addr: template_addr,
            target_addr: target_addr,
            envelope: envelope.clone(),
            context: context.clone(),
        }
    }

    /// New instance with explicit memory
    pub fn new_with_memory(
        memory: Memory,
        storage: AccountStorage,
        envelope: &Envelope,
        context: &Context,
        template_addr: TemplateAddr,
        target_addr: Address,
    ) -> Self {
        let func_env = Self::new(storage, envelope, context, template_addr, target_addr);
        func_env.borrow_mut().set_memory(memory);

        func_env
    }

    /// Returns the `Address` of the `Template` associated with the currently executed `Account`.
    pub fn template_addr(&self) -> &TemplateAddr {
        &self.template_addr
    }

    /// Returns the `Address` of the currently executed `Account` (a.k.a the `target`).
    pub fn target_addr(&self) -> &Address {
        &self.target_addr
    }

    /// Borrows the `FuncEnv`
    #[inline]
    pub fn borrow(&self) -> Ref<Inner> {
        self.inner.borrow()
    }

    /// Mutably Borrows the `FuncEnv`
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<Inner> {
        self.inner.borrow_mut()
    }
}

pub struct Inner {
    /// An accessor to the `Account`'s storage.
    pub storage: AccountStorage,

    /// Collected logs during execution.
    pub logs: Vec<ReceiptLog>,

    /// Pointer to `returndata`. Tuple stores `(offset, len)`.
    pub returndata: Option<(usize, usize)>,

    /// Instance's allocated memory.
    memory: Option<Memory>,

    /// Instance's amount of used space.
    used_memory: u64,

    /// Pointer to `calldata`. Tuple stores `(offset, len)`.
    calldata: Option<(usize, usize)>,
}

impl Inner {
    fn new(storage: AccountStorage) -> Self {
        let logs = Vec::new();

        Self {
            storage,
            logs,
            memory: None,
            calldata: None,
            returndata: None,
            used_memory: 0,
        }
    }

    pub fn set_calldata(&mut self, offset: usize, len: usize) {
        self.calldata = Some((offset, len));
    }

    pub fn calldata(&self) -> (usize, usize) {
        debug_assert!(self.calldata.is_some());

        self.calldata.unwrap()
    }

    pub fn set_returndata(&mut self, offset: usize, len: usize) {
        assert!(
            len > 0,
            "Can't set empty `returndata` (offset = {})",
            offset
        );

        debug_assert!(self.returndata.is_none());

        self.returndata = Some((offset, len));
    }

    pub fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }

    pub fn memory(&self) -> &Memory {
        debug_assert!(self.memory.is_some());

        self.memory.as_ref().unwrap()
    }

    pub fn set_used_memory(&mut self, used_memory: u64) {
        self.used_memory = used_memory;
    }

    pub fn used_memory(&self) -> u64 {
        self.used_memory
    }

    pub fn allocated_memory(&self) -> u64 {
        self.memory().data_size()
    }

    pub fn take_logs(&mut self) -> Vec<ReceiptLog> {
        std::mem::take(&mut self.logs)
    }
}
