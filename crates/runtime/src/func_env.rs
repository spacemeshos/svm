//! Implements [`FuncEnv`]. Used for managing data of running `Transaction`s.

use tokio::runtime::Runtime;
use wasmer::Memory;

use std::{
    collections::HashSet,
    rc::Rc,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use svm_state::AccountStorage;
use svm_types::{Address, Context, Envelope, ReceiptLog, TemplateAddr};

/// [`FuncEnv`] is a container for the accessible data by running [`Wasmer instance`](wasmer::Instance).
#[derive(wasmer::WasmerEnv, Clone)]
pub struct FuncEnv {
    pub tokio_runtime: Rc<TokioRuntime>,
    /// The [`TemplateAddr`] associated with the currently executed `Account`.
    pub template_addr: TemplateAddr,
    /// The [`Address`] of the currently executed `Account`.
    pub target_addr: Address,

    inner: Arc<RwLock<Inner>>,
    envelope: Envelope,
    context: Context,
}

impl FuncEnv {
    /// Creates a new instance
    pub fn new(
        tokio_runtime: Rc<TokioRuntime>,
        storage: AccountStorage,
        envelope: &Envelope,
        context: &Context,
        template_addr: TemplateAddr,
        target_addr: Address,
        mode: AccessMode,
    ) -> Self {
        let inner = Inner::new(storage);

        let env = Self {
            tokio_runtime,
            inner: Arc::new(RwLock::new(inner)),
            template_addr,
            target_addr,
            envelope: envelope.clone(),
            context: context.clone(),
        };
        env.set_access_mode(mode);

        env
    }

    /// New instance with explicit memory
    pub fn new_with_memory(
        tokio_runtime: Rc<TokioRuntime>,
        memory: Memory,
        storage: AccountStorage,
        envelope: &Envelope,
        context: &Context,
        template_addr: TemplateAddr,
        target_addr: Address,
        mode: AccessMode,
    ) -> Self {
        let env = Self::new(
            tokio_runtime,
            storage,
            envelope,
            context,
            template_addr,
            target_addr,
            mode,
        );
        env.borrow_mut().set_memory(memory);
        env
    }

    /// Borrows the `FuncEnv`
    #[inline]
    pub fn borrow(&self) -> RwLockReadGuard<Inner> {
        self.inner
            .read()
            .expect("Attempted read but RwLock is poisoned")
    }

    /// Mutably Borrows the `FuncEnv`
    #[inline]
    pub fn borrow_mut(&self) -> RwLockWriteGuard<Inner> {
        self.inner
            .write()
            .expect("Attempted write but RwLock is poisoned")
    }

    /// Sets the [`AccessMode`] and overrides the existing value.
    pub fn set_access_mode(&self, mode: AccessMode) {
        let mut borrow = self.borrow_mut();
        borrow.set_access_mode(mode);
    }

    /// Returns the current [`AccessMode`].
    pub fn access_mode(&self) -> AccessMode {
        let borrow = self.borrow();
        borrow.mode
    }
}

pub struct Inner {
    /// An accessor to the `Account`'s storage.
    storage: AccountStorage,

    /// Collected logs during execution.
    logs: Vec<ReceiptLog>,

    /// Pointer to `returndata`. Tuple stores `(offset, len)`.
    returndata: Option<(usize, usize)>,

    /// Instance's allocated memory.
    memory: Option<Memory>,

    /// Instance's amount of used space.
    used_memory: u64,

    /// Pointer to `calldata`. Tuple stores `(offset, len)`.
    calldata: Option<(usize, usize)>,

    /// The current [`AccessMode`] of the running transaction.
    mode: AccessMode,

    /// Set of [`Address`] that have been part of at least once `Coins Transfer` during transaction execution.
    touched_accounts: HashSet<Address>,
}

/// Denotes the capabilities allowed to the executing Account at a given point in time.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessMode {
    /// Access to [`AccountStorage`] is not allowed.
    AccessDenied,

    /// Only `Read Access` to [AccountStorage]'s `Immutable Storage` is allowed.
    ImmutableOnly,

    /// Full-Access to [`AccountStorage`] is allowed.
    FullAccess,
}

impl Inner {
    fn new(storage: AccountStorage) -> Self {
        let logs = Vec::new();
        let mut touched_accounts = HashSet::new();
        touched_accounts.insert(storage.address);

        Self {
            storage,
            logs,
            memory: None,
            calldata: None,
            returndata: None,
            used_memory: 0,
            mode: AccessMode::AccessDenied,
            touched_accounts,
        }
    }

    /// Adds a given account [`Address`] to the list of touched accounts.
    pub fn touch_account(&mut self, addr: Address) {
        self.touched_accounts.insert(addr);
    }

    pub fn touched_accounts(&self) -> HashSet<Address> {
        self.touched_accounts.clone()
    }

    pub fn set_access_mode(&mut self, mode: AccessMode) {
        self.mode = mode;
    }

    pub fn storage(&self) -> &AccountStorage {
        assert!(self.can_read());
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut AccountStorage {
        assert!(self.can_write());
        &mut self.storage
    }

    pub fn logs(&self) -> &[ReceiptLog] {
        &self.logs
    }

    pub fn logs_mut(&mut self) -> &mut Vec<ReceiptLog> {
        &mut self.logs
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

    pub fn returndata(&self) -> Option<(usize, usize)> {
        self.returndata
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

    #[inline]
    fn can_read(&self) -> bool {
        self.mode != AccessMode::AccessDenied
    }

    #[inline]
    fn can_write(&self) -> bool {
        matches!(self.mode, AccessMode::FullAccess)
    }
}
