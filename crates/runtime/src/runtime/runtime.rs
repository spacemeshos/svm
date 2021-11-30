use log::info;
use svm_codec::Codec;
use wasmer::{Instance, Module, WasmPtr, WasmTypeList};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use svm_gas::{FuncPrice, ProgramPricing};
use svm_hash::{Blake3Hasher, Hasher};
use svm_program::{Program, ProgramVisitor};
use svm_state::{AccountStorage, GlobalState, TemplateStorage};
use svm_types::{
    Address, BytesPrimitive, CallReceipt, Context, DeployReceipt, Envelope, Gas, GasMode, Layer,
    OOGError, ReceiptLog, RuntimeError, RuntimeFailure, Sections, SpawnAccount, SpawnReceipt,
    State, Template, TemplateAddr, Transaction,
};

use super::{Call, Function, Outcome};
use crate::error::ValidateError;
use crate::price_registry::PriceResolverRegistry;
use crate::{vmcalls, FuncEnv, ProtectedMode};

type OutcomeResult<T> = std::result::Result<Outcome<T>, RuntimeFailure>;
type Result<T> = std::result::Result<T, RuntimeFailure>;

const ERR_VALIDATE_SPAWN: &str = "Should have called `validate_spawn` first";
const ERR_VALIDATE_CALL: &str = "Should have called `validate_call` first";
const ERR_VALIDATE_DEPLOY: &str = "Should have called `validate_deploy` first";

/// An SVM runtime implementation based on [`Wasmer`](https://wasmer.io).
pub struct Runtime {
    /// Provided host functions to be consumed by running transactions.
    imports: (String, wasmer::Exports),
    gs: GlobalState,
    price_registry: PriceResolverRegistry,
    /// A naive cache for [`Template`]s' [`FuncPrice`]s. The cache key will, in
    /// the future, also include an identifier for which
    /// [`PriceResolver`](svm_gas::PriceResolver) should be used (possibly an
    /// `u16`?).
    template_prices: Rc<RefCell<HashMap<TemplateAddr, FuncPrice>>>,
}

impl Runtime {
    /// Initializes a new [`Runtime`].
    ///
    /// `template_prices` offers an easy way to inject an append-only, naive caching mechanism to
    /// the [`Template`] pricing logic; using a `None` will result in a new
    /// empty cache and on-the-fly calculation for all [`Template`]s.
    pub fn new(
        imports: (String, wasmer::Exports),
        global_state: GlobalState,
        price_registry: PriceResolverRegistry,
        template_prices: Option<Rc<RefCell<HashMap<TemplateAddr, FuncPrice>>>>,
    ) -> Self {
        let template_prices = template_prices.unwrap_or_default();

        Self {
            imports,
            gs: global_state,
            template_prices,
            price_registry,
        }
    }

    fn failure_to_receipt(&self, fail: RuntimeFailure) -> CallReceipt {
        CallReceipt::from_err(fail.err, fail.logs)
    }

    fn call_ctor(
        &mut self,
        spawn: &SpawnAccount,
        target: Address,
        gas_left: Gas,
        envelope: &Envelope,
        context: &Context,
    ) -> SpawnReceipt {
        let template = spawn.template_addr().clone();

        let call = Call {
            func_name: spawn.ctor_name(),
            func_input: spawn.ctor_data(),
            state: &State::zeros(),
            template,
            target: target.clone(),
            within_spawn: true,
            gas_limit: gas_left,
            protected_mode: ProtectedMode::FullAccess,
            envelope,
            context,
        };

        let receipt = self.exec_call::<(), ()>(&call);

        // TODO: move the `into_spawn_receipt` to a `From / TryFrom`
        svm_types::into_spawn_receipt(receipt, &target)
    }

    fn exec_call<'a, Args, Rets>(&'a mut self, call: &Call<'a>) -> CallReceipt {
        let result = self.exec::<(), (), _, _>(&call, |env, out| outcome_to_receipt(env, out));

        result.unwrap_or_else(|fail| self.failure_to_receipt(fail))
    }

    fn exec<Args, Rets, F, R>(&self, call: &Call, f: F) -> Result<R>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
        F: Fn(&FuncEnv, Outcome<Box<[wasmer::Val]>>) -> R,
    {
        let template = self.account_template(&call.target)?;

        let storage = AccountStorage::create(
            self.gs.clone(),
            &call.target,
            "NAME_TODO".to_string(),
            call.template,
            0,
            0,
        )
        .unwrap();

        let mut env = FuncEnv::new(
            storage,
            call.envelope,
            call.context,
            call.template.clone(),
            call.target.clone(),
            call.protected_mode,
        );

        let store = crate::wasm_store::new_store();
        let import_object = self.create_import_object(&store, &mut env);

        let res = self.run::<Args, Rets>(&call, &store, &env, &template, &import_object);
        res.map(|rets| f(&env, rets))
    }

    fn run<Args, Rets>(
        &self,
        call: &Call,
        store: &wasmer::Store,
        func_env: &FuncEnv,
        template: &Template,
        import_object: &wasmer::ImportObject,
    ) -> OutcomeResult<Box<[wasmer::Val]>>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        self.validate_call_contents(call, template, func_env)?;

        let module = self.compile_template(store, func_env, &template, call.gas_limit)?;
        let instance = self.instantiate(func_env, &module, import_object)?;

        set_memory(func_env, &instance);

        let func = self.func::<Args, Rets>(&instance, func_env, call.func_name)?;

        let mut out = if call.func_input.len() > 0 {
            self.call_with_alloc(&instance, func_env, call.func_input, &func, &[])?
        } else {
            self.wasmer_call(&instance, func_env, &func, &[])?
        };

        let logs = out.take_logs();

        match instance_gas_used(&instance) {
            Ok(gas_used) => {
                let returns = out.take_returns();
                let out = Outcome {
                    returns,
                    gas_used,
                    logs,
                };

                Ok(out)
            }
            Err(..) => Err(RuntimeFailure::new(RuntimeError::OOG, out.logs)),
        }
    }

    fn call_with_alloc<Args, Rets>(
        &self,
        instance: &Instance,
        env: &FuncEnv,
        calldata: &[u8],
        func: &Function<Args, Rets>,
        params: &[wasmer::Val],
    ) -> OutcomeResult<Box<[wasmer::Val]>>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        debug_assert!(calldata.is_empty() == false);

        let out = self.call_alloc(instance, env, calldata.len())?;

        // we assert that `svm_alloc` didn't touch the `returndata`
        // TODO: return an error instead of `panic`
        assert_no_returndata(env);

        let wasm_ptr = out.returns;
        set_calldata(env, calldata, wasm_ptr);

        self.wasmer_call(instance, env, func, params)
    }

    fn call_alloc(
        &self,
        instance: &Instance,
        env: &FuncEnv,
        size: usize,
    ) -> OutcomeResult<WasmPtr<u8>> {
        // Backups the current [`ProtectedMode`].
        let origin_mode = env.protected_mode();

        // Sets `Access Denied` mode while running `svm_alloc`.
        env.set_protected_mode(ProtectedMode::AccessDenied);

        let func_name = "svm_alloc";

        let func = self.func::<u32, u32>(&instance, env, func_name);
        if func.is_err() {
            // ### Notes:
            //
            // We don't restore the original [`ProtectedMode`]
            // since `svm_alloc` has failed and the transaction will halt.
            let err = err::func_not_found(env, func_name);
            return Err(err);
        }

        let func = func.unwrap();
        let params: [wasmer::Val; 1] = [(size as i32).into()];

        let out = self
            .wasmer_call(instance, env, &func, &params)?
            .map(|rets| {
                let ret = &rets[0];
                let offset = ret.i32().unwrap() as u32;

                WasmPtr::new(offset)
            });

        // Restores the original [`ProtectedMode`].
        env.set_protected_mode(origin_mode);

        Ok(out)
    }

    fn wasmer_call<Args, Rets>(
        &self,
        instance: &Instance,
        env: &FuncEnv,
        func: &Function<Args, Rets>,
        params: &[wasmer::Val],
    ) -> OutcomeResult<Box<[wasmer::Val]>>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let wasmer_func = func.wasmer_func();
        let returns = wasmer_func.call(params);
        let logs = env.borrow_mut().take_logs();

        if returns.is_err() {
            let err = err::func_failed(env, func.name(), returns.unwrap_err(), logs);
            return Err(err);
        }

        match instance_gas_used(&instance) {
            Ok(gas_used) => {
                let out = Outcome::new(returns.unwrap(), gas_used, logs);
                Ok(out)
            }
            Err(..) => {
                let err = RuntimeFailure::new(RuntimeError::OOG, logs);
                Err(err)
            }
        }
    }

    fn instantiate(
        &self,
        env: &FuncEnv,
        module: &Module,
        import_object: &wasmer::ImportObject,
    ) -> Result<Instance> {
        info!("Runtime `instantiate` (using Wasmer `Instance#new`)");

        let instance = Instance::new(module, import_object);
        instance.map_err(|err| err::instantiation_failed(env, err))
    }

    fn func<'i, Args, Rets>(
        &self,
        instance: &'i Instance,
        env: &FuncEnv,
        func_name: &'i str,
    ) -> Result<Function<'i, Args, Rets>>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let func = instance.exports.get_function(func_name);
        if func.is_err() {
            let err = err::func_not_found(env, func_name);
            return Err(err);
        }

        let func = func.unwrap();
        let native = func.native::<Args, Rets>();

        if native.is_err() {
            let err = err::func_invalid_sig(env, func_name);
            return Err(err);
        }

        let func = Function::new(func, func_name);
        Ok(func)
    }

    fn create_import_object(
        &self,
        store: &wasmer::Store,
        env: &mut FuncEnv,
    ) -> wasmer::ImportObject {
        let mut import_object = wasmer::ImportObject::new();

        // Registering SVM internals
        let mut internals = wasmer::Exports::new();
        vmcalls::wasmer_register(store, env, &mut internals);
        import_object.register("svm", internals);

        // Registering the externals provided to the Runtime
        let (name, exports) = &self.imports;
        debug_assert_ne!(name, "svm");

        import_object.register(name, exports.clone());

        import_object
    }

    fn account_template(
        &self,
        account_addr: &Address,
    ) -> std::result::Result<Template, RuntimeError> {
        let accounts = AccountStorage::load(self.gs.clone(), account_addr).unwrap();
        let template_addr = accounts.template_addr().unwrap();
        let template_storage = TemplateStorage::load(self.gs.clone(), &template_addr).unwrap();
        let sections = template_storage.sections().unwrap();

        // TODO: Only fetch core sections.
        Ok(Template::from_sections(sections))
    }

    fn compile_template(
        &self,
        store: &wasmer::Store,
        env: &FuncEnv,
        template: &Template,
        gas_left: Gas,
    ) -> std::result::Result<Module, RuntimeFailure> {
        let module_res = Module::from_binary(store, template.code());
        let _gas_left = gas_left.unwrap_or(0);

        module_res.map_err(|err| err::compilation_failed(env, err))
    }

    fn validate_call_contents(
        &self,
        call: &Call,
        template: &Template,
        env: &FuncEnv,
    ) -> std::result::Result<(), RuntimeFailure> {
        // TODO: validate there is enough gas for running the `Transaction`.
        // * verify
        // * call
        // * other factors

        let spawning = call.within_spawn;
        let ctor = template.is_ctor(call.func_name);

        if spawning && !ctor {
            let msg = "expected function to be a constructor";
            let err = err::func_not_allowed(env, call.func_name, msg);

            return Err(err);
        }

        if !spawning && ctor {
            let msg = "expected function to be a non-constructor";
            let err = err::func_not_allowed(env, call.func_name, msg);

            return Err(err);
        }

        Ok(())
    }

    fn build_call<'a>(
        &self,
        tx: &'a Transaction,
        envelope: &'a Envelope,
        context: &'a Context,
        protected_mode: ProtectedMode,
        func_name: &'a str,
        func_input: &'a [u8],
    ) -> Call<'a> {
        let target = tx.target();
        let account_storage = AccountStorage::load(self.gs.clone(), target).unwrap();
        let template = account_storage.template_addr().unwrap();

        Call {
            func_name,
            func_input,
            target: target.clone(),
            template,
            state: context.state(),
            gas_limit: envelope.gas_limit(),
            protected_mode,
            within_spawn: false,
            envelope,
            context,
        }
    }

    /// Returns the state root hash and layer ID of the last layer.
    pub fn current_layer(&mut self) -> (Layer, State) {
        self.gs.current_layer().unwrap()
    }

    /// Increases the balance by a given amount associated with `account_addr`.
    pub fn increase_balance(&mut self, account_addr: &Address, amount: u64) -> Result<()> {
        let mut accounts = AccountStorage::load(self.gs.clone(), account_addr).unwrap();
        let balance = accounts.balance().unwrap();
        let new_balance = balance
            .checked_add(amount)
            .expect("Overflow when increasing balance.");
        accounts.set_balance(new_balance).unwrap();

        Ok(())
    }

    /// Creates a new account at genesis with the given information.
    pub fn create_account(
        &mut self,
        account_addr: &Address,
        name: String,
        balance: u64,
        counter: u128,
    ) -> Result<()> {
        AccountStorage::create(
            self.gs.clone(),
            account_addr,
            name,
            TemplateAddr::god_template(),
            balance,
            counter,
        )
        .unwrap();

        Ok(())
    }

    /// Validates syntactically a binary `Deploy Template` message prior to executing it.
    pub fn validate_deploy(&self, message: &[u8]) -> std::result::Result<(), ValidateError> {
        let mut cursor = std::io::Cursor::new(message);
        let template = svm_codec::template::decode(&mut cursor, None)?;
        let code = template.code();

        // Opcode and `svm_alloc` checks should only ever be run when deploying [`Template`]s.
        // There's no reason to also do it when spawning new `Account`
        // over already-validated [`Template`]s
        let program = Program::new(code, true)?;
        svm_gas::validate_wasm(&program, false)?;

        Ok(())
    }

    /// Validates syntactically a binary `Spawn Account` message prior to executing it.
    pub fn validate_spawn(&self, message: &[u8]) -> std::result::Result<(), ValidateError> {
        SpawnAccount::decode_bytes(message)?;
        Ok(())
    }

    /// Validates syntactically a binary `Call Account` message prior to executing it.
    pub fn validate_call(&self, message: &[u8]) -> std::result::Result<(), ValidateError> {
        Transaction::decode_bytes(message)?;
        Ok(())
    }

    /// Deploys a `Template`
    pub fn deploy(
        &mut self,
        envelope: &Envelope,
        message: &[u8],
        _context: &Context,
    ) -> DeployReceipt {
        info!("Runtime `deploy`");

        let sections = Sections::decode_bytes(message).expect(ERR_VALIDATE_DEPLOY);
        let template = Template::from_sections(sections);

        let gas_limit = envelope.gas_limit();
        let install_price = svm_gas::transaction::deploy(message);

        if gas_limit < install_price {
            return DeployReceipt::new_oog();
        }

        let gas_used = Gas::with(install_price);
        let addr = compute_template_addr(&template);

        TemplateStorage::create(
            self.gs.clone(),
            &addr,
            template.sections().clone(),
            template.sections().clone(),
        )
        .unwrap();

        DeployReceipt::new(addr, gas_used)
    }

    /// Spawns a new `Account`
    pub fn spawn(
        &mut self,
        envelope: &Envelope,
        message: &[u8],
        context: &Context,
    ) -> SpawnReceipt {
        // TODO: refactor this function (it has got a bit lengthy...)

        info!("Runtime `spawn`");

        let gas_limit = envelope.gas_limit();
        let base = SpawnAccount::decode_bytes(message).expect(ERR_VALIDATE_SPAWN);

        let template_addr = base.account.template_addr();

        // TODO: load only the `Sections` relevant for spawning
        let template_storage = TemplateStorage::load(self.gs.clone(), template_addr).unwrap();
        let sections = template_storage.sections().unwrap();
        let template = Template::from_sections(sections);

        let code_section = template.code_section();
        let code = code_section.code();
        let gas_mode = code_section.gas_mode();
        let program = Program::new(code, false).unwrap();

        // We're using a naive memoization mechanism: we only ever add, never
        // remove. This means there's no cache invalidation at all. We can
        // easily afford to do this because the number of templates that exist
        // at genesis is fixed and won't grow.
        let mut template_prices = self.template_prices.borrow_mut();
        let func_price = if let Some(prices) = template_prices.get(&template_addr) {
            prices
        } else {
            let pricer = self
                .price_registry
                .get(0)
                .expect("Missing pricing utility.");
            let program_pricing = ProgramPricing::new(pricer);
            let prices = program_pricing.visit(&program).unwrap();

            template_prices.insert(template_addr.clone(), prices);
            template_prices.get(template_addr).unwrap()
        };

        let spawn = base;

        if !template.is_ctor(spawn.ctor_name()) {
            // The [`Template`] is faulty.
            let account = spawn.account();
            let account_addr = compute_account_addr(&spawn);

            return SpawnReceipt::from_err(
                RuntimeError::FuncNotAllowed {
                    target: account_addr,
                    template: account.template_addr().clone(),
                    func: spawn.ctor_name().to_string(),
                    msg: "The given function is not a `ctor`.".to_string(),
                },
                vec![],
            );
        }

        match gas_mode {
            GasMode::Fixed => {
                let ctor_func_index = program.exports().get(spawn.ctor_name()).unwrap();
                let price = func_price.get(ctor_func_index) as u64;
                if gas_limit <= price {
                    return SpawnReceipt::new_oog(vec![]);
                }
            }
            GasMode::Metering => unreachable!("Not supported yet... (TODO)"),
        }

        // We don't need this anymore!
        drop(template_prices);

        let payload_price = svm_gas::transaction::spawn(message);
        let gas_left = gas_limit - payload_price;

        match gas_left {
            Ok(gas_left) => {
                let account = spawn.account();
                let target = compute_account_addr(&spawn);

                AccountStorage::create(
                    self.gs.clone(),
                    &target,
                    account.name().to_string(),
                    account.template_addr().clone(),
                    0,
                    0,
                )
                .unwrap();

                self.call_ctor(&spawn, target, gas_left, envelope, context)
            }
            Err(..) => SpawnReceipt::new_oog(Vec::new()),
        }
    }

    /// Verifies a [`Transaction`](svm_types::Transaction) before execution.
    pub fn verify(
        &mut self,
        envelope: &Envelope,
        message: &[u8],
        context: &Context,
    ) -> CallReceipt {
        let tx = Transaction::decode_bytes(message).expect(ERR_VALIDATE_CALL);

        // ### Important:
        //
        // Right now we disallow any `Storage` access while running `svm_verify`.
        // This hard restriction might be mitigated in future versions.
        //
        // In that case, the current behavior should be backward-compatible since
        // we could always executed `Access Denied` logic when partial `Storage` access will be allowed by SVM.
        let call = self.build_call(
            &tx,
            envelope,
            context,
            ProtectedMode::AccessDenied,
            "svm_verify",
            tx.verifydata(),
        );

        // TODO: override the `call.gas_limit` with `VERIFY_MAX_GAS`
        self.exec_call::<(), ()>(&call)
    }

    /// Executes a [`Transaction`](svm_types::Transaction) and returns its output [`CallReceipt`].
    ///
    /// This function should be called only if the `verify` stage has passed.
    pub fn call(&mut self, envelope: &Envelope, message: &[u8], context: &Context) -> CallReceipt {
        let tx = Transaction::decode_bytes(message).expect(ERR_VALIDATE_CALL);

        let call = self.build_call(
            &tx,
            envelope,
            context,
            ProtectedMode::FullAccess,
            tx.func_name(),
            tx.calldata(),
        );

        self.exec_call::<(), ()>(&call)
    }

    /// Moves the internal state of this [`Runtime`] back to the time of
    /// `layer_id`.
    pub fn rewind(&mut self, layer_id: Layer) -> Result<()> {
        self.gs
            .rewind(layer_id)
            .map_err(|_e| RuntimeFailure::new(RuntimeError::OOG, vec![]))
    }

    /// Creates a new layer with the given changes.
    pub fn commit(&mut self) -> Result<()> {
        self.gs
            .commit()
            .map_err(|_e| RuntimeFailure::new(RuntimeError::OOG, vec![]))?;
        Ok(())
    }

    /// Given the address of an account, it attempts to read:
    ///
    /// - balance;
    /// - counter;
    /// - template's address;
    ///
    /// from the database layer.
    pub fn get_account(&self, account_addr: &Address) -> Option<(u64, u128, TemplateAddr)> {
        let account_storage = AccountStorage::load(self.gs.clone(), account_addr).unwrap();
        let balance = account_storage.balance().unwrap();
        let counter = account_storage.counter().unwrap();
        let template_addr = account_storage.template_addr().unwrap();
        Some((balance, counter, template_addr))
    }

    /// Sends coins from the current executing account to a destination account.
    ///
    /// # Panics
    ///
    /// Panics when the destination account does not exist.
    pub fn transfer(&self, src_addr: &Address, dst_addr: &Address, amount: u64) {
        let mut src_account = AccountStorage::load(self.gs.clone(), src_addr).unwrap();

        let mut dst_account = if let Some((_bal, _counter, _addr)) = self.get_account(dst_addr) {
            AccountStorage::load(self.gs.clone(), dst_addr).unwrap()
        } else {
            panic!("Destination account does not exist")
        };

        let src_bal = src_account.balance().unwrap();
        let dst_bal = dst_account.balance().unwrap();

        if src_bal < amount {
            panic!("Not enough balance to execute transfer");
        }
        src_account
            .set_balance(src_bal.checked_sub(amount).unwrap())
            .unwrap();
        dst_account
            .set_balance(dst_bal.checked_add(amount).unwrap())
            .unwrap();
    }
}

fn compute_template_addr(template: &Template) -> TemplateAddr {
    let hash = Blake3Hasher::hash(template.code());

    TemplateAddr::new(&hash[..TemplateAddr::N])
}

fn compute_account_addr(spawn: &SpawnAccount) -> Address {
    let template_addr = spawn.template_addr();
    let hash = Blake3Hasher::hash(template_addr.as_slice());

    Address::new(&hash[..Address::N])
}

fn read_memory(env: &FuncEnv, offset: usize, length: usize) -> Vec<u8> {
    assert!(length > 0);

    let borrow = env.borrow();
    let memory = borrow.memory();

    let view = memory.view::<u8>();
    assert!(view.len() > offset + length - 1);

    let cells = &view[offset..(offset + length)];
    cells.iter().map(|c| c.get()).collect()
}

fn set_memory(env: &FuncEnv, instance: &Instance) {
    // TODO: raise when no exported memory exists
    let memory = instance.exports.get_memory("memory").unwrap();

    env.borrow_mut().set_memory(memory.clone());
}

fn set_calldata(env: &FuncEnv, calldata: &[u8], wasm_ptr: WasmPtr<u8>) {
    debug_assert!(calldata.is_empty() == false);

    let (offset, len) = {
        let borrow = env.borrow();
        let memory = borrow.memory();

        // Each WASM instance memory contains at least one `WASM Page`. (A `Page` size is 64KB)
        // The `len(calldata)` will be less than the `WASM Page` size.
        //
        // In any case, the `alloc_memory` is in charge of allocating enough memory
        // for the program to run (so we don't need to have any bounds-checking here).
        //
        // TODO: add to `validate_template` checking that `calldata` doesn't exceed ???
        // (we'll need to decide on a `calldata` limit).
        //
        // See [issue #140](https://github.com/spacemeshos/svm/issues/140)
        let offset = wasm_ptr.offset() as usize;
        let length = calldata.len();
        let view = memory.view::<u8>();

        // TODO: fail safely, instead of using `assert!`
        assert!(view.len() > offset + length - 1);

        let cells = &view[offset..(offset + length)];
        for (cell, &byte) in cells.iter().zip(calldata.iter()) {
            cell.set(byte);
        }

        (offset, length)
    };

    env.borrow_mut().set_calldata(offset, len);
}

fn commit_changes(env: &FuncEnv) -> State {
    let mut borrow = env.borrow_mut();
    let storage = borrow.storage_mut();
    storage.gs.checkpoint().unwrap();
    storage.gs.commit().unwrap().1.into()
}

fn outcome_to_receipt(env: &FuncEnv, mut out: Outcome<Box<[wasmer::Val]>>) -> CallReceipt {
    CallReceipt {
        version: 0,
        success: true,
        error: None,
        returndata: Some(take_returndata(env)),
        new_state: Some(commit_changes(&env)),
        gas_used: out.gas_used(),
        logs: out.take_logs(),
    }
}

fn assert_no_returndata(env: &FuncEnv) {
    assert!(env.borrow().returndata().is_none())
}

fn take_returndata(env: &FuncEnv) -> Vec<u8> {
    let data = env.borrow().returndata();

    match data {
        Some((offset, length)) => read_memory(env, offset, length),
        None => Vec::new(),
    }
}

/// Calculates the amount of gas used by `instance`.
fn instance_gas_used(_instance: &Instance) -> std::result::Result<Gas, OOGError> {
    // TODO: read `gas_used` out of `instance`
    Ok(Gas::new())
}

mod err {
    use super::*;

    pub fn func_not_found(env: &FuncEnv, func_name: &str) -> RuntimeFailure {
        RuntimeError::FuncNotFound {
            target: env.target_addr().clone(),
            template: env.template_addr().clone(),
            func: func_name.to_string(),
        }
        .into()
    }

    pub fn instantiation_failed(env: &FuncEnv, err: wasmer::InstantiationError) -> RuntimeFailure {
        RuntimeError::InstantiationFailed {
            target: env.target_addr().clone(),
            template: env.template_addr().clone(),
            msg: err.to_string(),
        }
        .into()
    }

    pub fn func_not_allowed(env: &FuncEnv, func_name: &str, msg: &str) -> RuntimeFailure {
        RuntimeError::FuncNotAllowed {
            target: env.target_addr().clone(),
            template: env.template_addr().clone(),
            func: func_name.to_string(),
            msg: msg.to_string(),
        }
        .into()
    }

    pub fn func_invalid_sig(env: &FuncEnv, func_name: &str) -> RuntimeFailure {
        RuntimeError::FuncInvalidSignature {
            target: env.target_addr().clone(),
            template: env.template_addr().clone(),
            func: func_name.to_string(),
        }
        .into()
    }

    pub fn func_failed(
        env: &FuncEnv,
        func_name: &str,
        err: wasmer::RuntimeError,
        logs: Vec<ReceiptLog>,
    ) -> RuntimeFailure {
        let err = RuntimeError::FuncFailed {
            target: env.target_addr().clone(),
            template: env.template_addr().clone(),
            func: func_name.to_string(),
            msg: err.to_string(),
        };

        RuntimeFailure::new(err, logs)
    }

    pub fn compilation_failed(env: &FuncEnv, err: wasmer::CompileError) -> RuntimeFailure {
        RuntimeError::CompilationFailed {
            target: env.target_addr().clone(),
            template: env.template_addr().clone(),
            msg: err.to_string(),
        }
        .into()
    }
}