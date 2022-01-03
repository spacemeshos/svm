use log::info;
use svm_codec::Codec;
use wasmer::{Instance, Module, WasmPtr, WasmTypeList};

use svm_hash::{Blake3Hasher, Hasher};
use svm_program::Program;
use svm_state::{AccountStorage, GlobalState, TemplateStorage};
use svm_types::{
    Address, BytesPrimitive, CallReceipt, Context, DeployReceipt, Envelope, Gas, GasMode, Layer,
    OOGError, ReceiptLog, RuntimeError, RuntimeFailure, SectionKind, Sections, SpawnAccount,
    SpawnReceipt, State, Template, TemplateAddr, Transaction,
};

use super::gas_tank::GasTank;
use super::{Call, Function, Outcome, TemplatePriceCache};
use crate::error::ValidateError;
use crate::{vmcalls, AccessMode, FuncEnv};

type OutcomeResult<T> = std::result::Result<Outcome<T>, RuntimeFailure>;
type Result<T> = std::result::Result<T, RuntimeFailure>;

const ERR_VALIDATE_SPAWN: &str = "Should have called `validate_spawn` first";
const ERR_VALIDATE_CALL: &str = "Should have called `validate_call` first";
const ERR_VALIDATE_DEPLOY: &str = "Should have called `validate_deploy` first";

/// An SVM runtime implementation based on [`Wasmer`](https://wasmer.io).
pub struct Runtime {
    /// The [`GlobalState`]
    gs: GlobalState,

    /// A Cache for a [`Template`]'= functions prices.
    template_price: TemplatePriceCache,
}

impl Runtime {
    /// Initializes a new [`Runtime`].
    pub fn new(gs: GlobalState, template_price: TemplatePriceCache) -> Self {
        Self { gs, template_price }
    }

    fn call_ctor(
        &mut self,
        spawn: &SpawnAccount,
        target: Address,
        envelope: &Envelope,
        context: &Context,
        gas_left: GasTank,
    ) -> SpawnReceipt {
        let template = spawn.template_addr().clone();

        let call = Call {
            func_name: spawn.ctor_name(),
            func_input: spawn.ctor_data(),
            state: &State::zeros(),
            template_addr: template,
            target: target.clone(),
            within_spawn: true,
            gas_left,
            access_mode: AccessMode::FullAccess,
            envelope,
            context,
        };

        let mut receipt = self.exec_call::<(), ()>(&call);
        receipt.touched_accounts.insert(target);
        receipt
            .touched_accounts
            .insert(envelope.principal().clone());

        // TODO: move the `into_spawn_receipt` to a `From / TryFrom`
        svm_types::into_spawn_receipt(receipt, &target)
    }

    fn exec_call<'a, Args, Rets>(&'a mut self, call: &Call<'a>) -> CallReceipt {
        let result = self.exec::<(), (), _, _>(&call, |env, out| outcome_to_receipt(env, out));
        result.unwrap_or_else(|fail| CallReceipt::from_err(fail.err, fail.logs))
    }

    fn exec<Args, Rets, F, R>(&self, call: &Call, f: F) -> Result<R>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
        F: Fn(&FuncEnv, Outcome<Box<[wasmer::Val]>>) -> R,
    {
        let template = self.account_template(&call.target)?;
        let storage = AccountStorage::load(self.gs.clone(), &call.target).unwrap();

        let mut env = FuncEnv::new(
            storage,
            call.envelope,
            call.context,
            call.template_addr.clone(),
            call.target.clone(),
            call.access_mode,
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
        self.validate_func_usage(call, template, func_env)?;

        let module = self.compile_template(store, func_env, &template, call.gas_left)?;
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
            Err(_) => Err(RuntimeFailure::new(RuntimeError::OOG, out.logs)),
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
        // Backups the current [`AccessMode`].
        let origin_mode = env.access_mode();

        // Sets `Access Denied` mode while running `svm_alloc`.
        env.set_access_mode(AccessMode::AccessDenied);

        let func_name = "svm_alloc";

        let func = self.func::<u32, u32>(&instance, env, func_name);
        if func.is_err() {
            // ### Notes:
            //
            // We don't restore the original [`AccessMode`]
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

        // Restores the original [`AccessMode`].
        env.set_access_mode(origin_mode);

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

        // Registering SVM host functions.
        let mut exports = wasmer::Exports::new();
        vmcalls::wasmer_register(store, env, &mut exports);
        import_object.register("svm", exports);
        import_object
    }

    fn account_template(
        &self,
        account_addr: &Address,
    ) -> std::result::Result<Template, RuntimeFailure> {
        // TODO:
        //
        // * Return a `RuntimeFailure` when `account_addr` doesn't exist.
        let account = AccountStorage::load(self.gs.clone(), account_addr).unwrap();
        let template_addr = account.template_addr().unwrap();
        self.load_template(&template_addr)
    }

    fn load_template(
        &self,
        template_addr: &TemplateAddr,
    ) -> std::result::Result<Template, RuntimeFailure> {
        let template_storage = TemplateStorage::load(self.gs.clone(), &template_addr).unwrap();
        let sections = template_storage.sections().unwrap();
        let template = Template::from_sections(sections);

        // TODO:
        //
        // * Return a `RuntimeFailure` when `Template` doesn't exist.
        // * Fetch only the `Core Sections`.
        // * Add `non_core` sections to be fetched as a param (optional)
        Ok(template)
    }

    fn compile_template(
        &self,
        store: &wasmer::Store,
        env: &FuncEnv,
        template: &Template,
        _gas_left: GasTank,
    ) -> std::result::Result<Module, RuntimeFailure> {
        let module_res = Module::from_binary(store, template.code());
        module_res.map_err(|err| err::compilation_failed(env, err))
    }

    fn validate_func_usage(
        &self,
        call: &Call,
        template: &Template,
        env: &FuncEnv,
    ) -> std::result::Result<(), RuntimeFailure> {
        // TODO: validate there is enough gas for running the `Transaction`.
        // * verify
        // * call
        // * other factors

        if call.within_spawn {
            self.ensure_ctor(&call.template_addr, template, &call.func_name)
        } else {
            self.ensure_not_ctor(template, env, call)
        }
    }

    fn ensure_ctor(
        &self,
        template_addr: &TemplateAddr,
        template: &Template,
        func_name: &str,
    ) -> std::result::Result<(), RuntimeFailure> {
        debug_assert!(template.contains(SectionKind::Ctors));

        if template.is_ctor(func_name) {
            Ok(())
        } else {
            let err = err::func_not_ctor(template_addr, func_name);
            Err(err)
        }
    }

    fn ensure_not_ctor(
        &self,
        template: &Template,
        env: &FuncEnv,
        call: &Call,
    ) -> std::result::Result<(), RuntimeFailure> {
        if template.is_ctor(call.func_name) {
            let msg = "expected function not to be a constructor";
            let err = err::func_not_allowed(env, call.func_name, msg);
            Err(err)
        } else {
            Ok(())
        }
    }

    fn check_gas_for_payload(
        &self,
        _envelope: &Envelope,
        message: &[u8],
        gas_left: GasTank,
    ) -> GasTank {
        if gas_left.is_empty() {
            return GasTank::Empty;
        }

        // TODO: take into account the `Envelope` as well (not only the `Message`)
        let payload_price = svm_gas::transaction::spawn(message);
        gas_left.consume(payload_price)
    }

    fn check_gas_for_func(
        &self,
        template_addr: &TemplateAddr,
        template: &Template,
        func_name: &str,
        gas_left: GasTank,
    ) -> GasTank {
        if gas_left.is_empty() {
            return GasTank::Empty;
        }

        let code_section = template.code_section();
        let code = code_section.code();
        let gas_mode = code_section.gas_mode();
        let program = Program::new(code, false).unwrap();

        match gas_mode {
            GasMode::Fixed => {
                let func_index = program.exports().get(func_name).unwrap();
                let func_price = self.template_price.price_of(&template_addr, &program);

                let price = func_price.get(func_index) as u64;
                gas_left.consume(price)
            }
            GasMode::Metering => unreachable!("Not supported yet... (TODO)"),
        }
    }

    fn build_call<'a>(
        &self,
        tx: &'a Transaction,
        envelope: &'a Envelope,
        context: &'a Context,
        access_mode: AccessMode,
        func_name: &'a str,
        func_input: &'a [u8],
        gas_left: GasTank,
    ) -> Call<'a> {
        let target = tx.target();
        let account_storage = AccountStorage::load(self.gs.clone(), target).unwrap();
        let template = account_storage.template_addr().unwrap();

        Call {
            func_name,
            func_input,
            target: target.clone(),
            template_addr: template,
            state: context.state(),
            gas_left,
            access_mode,
            within_spawn: false,
            envelope,
            context,
        }
    }

    /// Returns the [`State`] root hash and [`Layer`] of the last layer.
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

    /// Creates a new `Genesis Account`.
    pub fn create_genesis_account(
        &mut self,
        account_addr: &Address,
        name: impl ToString,
        balance: u64,
        counter: u128,
    ) -> Result<()> {
        self.create_account(
            account_addr,
            TemplateAddr::god_template(),
            name.to_string(),
            balance,
            counter,
        )
        .unwrap();

        Ok(())
    }

    /// Creates a new `Account` with the given params.
    pub fn create_account(
        &mut self,
        account_addr: &Address,
        template_addr: TemplateAddr,
        name: String,
        balance: u64,
        counter: u128,
    ) -> Result<()> {
        AccountStorage::create(
            self.gs.clone(),
            account_addr,
            name,
            template_addr,
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

        let gas_left = envelope.gas_limit();
        let deploy_price = svm_gas::transaction::deploy(message);

        if gas_left < deploy_price {
            return DeployReceipt::new_oog();
        }

        let gas_used = Gas::with(deploy_price);
        let addr = svm_types::compute_template_addr(template.code_section());

        // TODO:
        //
        // * Create a `Deploy Section` to be added to `TemplateStorage`
        // * Have `template.core_sections() and `template.noncore_sections()`
        // * Pass to `TemplateStorage` `core sections` and `non-core sections`
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
        info!("Runtime `spawn`");

        let gas_left = GasTank::new(envelope.gas_limit());
        let spawn = SpawnAccount::decode_bytes(message).expect(ERR_VALIDATE_SPAWN);
        let template_addr = spawn.account.template_addr();
        let template = self.load_template(&template_addr);

        if let Err(fail) = template {
            return SpawnReceipt::from_err(fail.err, fail.logs);
        }

        let template = template.unwrap();
        let ctor = spawn.ctor_name();

        if let Err(fail) = self.ensure_ctor(&template_addr, &template, ctor) {
            return SpawnReceipt::from_err(fail.err, fail.logs);
        }

        let gas_left = self.check_gas_for_payload(envelope, message, gas_left);
        let gas_left = self.check_gas_for_func(&template_addr, &template, ctor, gas_left);

        if gas_left.is_empty() {
            return SpawnReceipt::new_oog(Vec::new());
        }

        let account = spawn.account();
        let target = compute_account_addr(&spawn);

        self.create_account(
            &target,
            template_addr.clone(),
            account.name().to_string(),
            0,
            0,
        )
        .unwrap();

        self.call_ctor(&spawn, target, envelope, context, gas_left)
    }

    /// Verifies a [`Transaction`](svm_types::Transaction) before execution.
    pub fn verify(
        &mut self,
        envelope: &Envelope,
        message: &[u8],
        context: &Context,
    ) -> CallReceipt {
        let tx = Transaction::decode_bytes(message).expect(ERR_VALIDATE_CALL);
        let gas_left = GasTank::new(envelope.gas_limit());

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
            AccessMode::AccessDenied,
            "svm_verify",
            tx.verifydata(),
            gas_left,
        );

        // TODO: override the `call.gas_limit` with `VERIFY_MAX_GAS`
        self.exec_call::<(), ()>(&call)
    }

    /// Executes a [`Transaction`](svm_types::Transaction) and returns its output [`CallReceipt`].
    ///
    /// This function should be called only if the `verify` stage has passed.
    pub fn call(&mut self, envelope: &Envelope, message: &[u8], context: &Context) -> CallReceipt {
        let tx = Transaction::decode_bytes(message).expect(ERR_VALIDATE_CALL);
        let gas_left = GasTank::new(envelope.gas_limit());

        let call = self.build_call(
            &tx,
            envelope,
            context,
            AccessMode::FullAccess,
            tx.func_name(),
            tx.calldata(),
            gas_left,
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

    /// Returns [`true`] iff the underlying [`GlobalState`] has changed since
    /// the last call to [`Runtime::commit`].
    pub fn has_uncommitted_changes(&self) -> Result<bool> {
        Ok(self.gs.has_uncommitted_changes())
    }

    /// Given the address of an account, it attempts to read:
    ///
    /// - balance;
    /// - counter;
    /// - template's address;
    ///
    /// from the database layer.
    pub fn get_account(&self, account_addr: &Address) -> Option<(u64, u128, TemplateAddr)> {
        let account_storage = AccountStorage::load(self.gs.clone(), account_addr).ok()?;
        let balance = account_storage.balance().ok()?;
        let counter = account_storage.counter().ok()?;
        let template_addr = account_storage.template_addr().ok()?;
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

/// Calculates the address of a newly spawned account based on its template.
pub fn compute_account_addr(spawn: &SpawnAccount) -> Address {
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
        touched_accounts: env.borrow().touched_accounts(),
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

    pub fn func_not_ctor(template_addr: &TemplateAddr, func_name: &str) -> RuntimeFailure {
        RuntimeError::FuncNotCtor {
            template: template_addr.clone(),
            func: func_name.to_string(),
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
