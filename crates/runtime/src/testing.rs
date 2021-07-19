//! Implements common functionality to be consumed by tests.

use wasmer::{ImportObject, Instance, Module, Store};

use std::cell::RefCell;
use std::rc::Rc;

use svm_codec::api::builder::{SpawnBuilder, TemplateBuilder, TxBuilder};
use svm_codec::template;
use svm_layout::{FixedLayout, Layout};
use svm_storage::{
    account::{AccountKVStore, AccountStorage},
    kv::{FakeKV, StatefulKV},
};
use svm_types::{
    AccountAddr, Address, CodeSection, CtorsSection, DataSection, Gas, HeaderSection, State,
    TemplateAddr,
};

use crate::env::{DefaultMemAccountStore, DefaultMemEnvTypes, DefaultMemTemplateStore};
use crate::storage::StorageBuilderFn;
use crate::{Config, DefaultRuntime, Env};

/// Hold a Wasm file in textual or binary form
pub enum WasmFile<'a> {
    /// Textual Wasm
    Text(&'a str),

    /// Binary Wasm
    Binary(&'a [u8]),
}

impl<'a> WasmFile<'a> {
    fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Text(text) => wat::parse_str(text).unwrap(),
            Self::Binary(wasm) => wasm.to_vec(),
        }
    }
}

impl<'a> From<&'a str> for WasmFile<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(text)
    }
}

impl<'a> From<&'a [u8]> for WasmFile<'a> {
    fn from(wasm: &'a [u8]) -> Self {
        Self::Binary(wasm)
    }
}

/// Compiles a wasm program in text format (a.k.a WAST) into a `Module` (`wasmer`)
pub fn wasmer_compile(store: &Store, wasm_file: WasmFile, _gas_limit: Gas) -> Module {
    let wasm = wasm_file.into_bytes();

    Module::from_binary(&store, &wasm[..]).unwrap()
}

/// Instantiate a `wasmer` instance
pub fn wasmer_instantiate(
    store: &Store,
    import_object: &ImportObject,
    wasm_file: WasmFile,
    gas_limit: Gas,
) -> Instance {
    let module = wasmer_compile(store, wasm_file, gas_limit);

    Instance::new(&module, import_object).unwrap()
}

/// Given an `Account` `Address` and its `layout`, it initializes a new blank [`AccountStorage`].
pub fn blank_storage(account_addr: &Address, layout: &FixedLayout) -> AccountStorage {
    let state_kv = memory_state_kv_init();
    let account_kv = AccountKVStore::new(account_addr.clone(), &state_kv);

    AccountStorage::new(layout.clone(), account_kv)
}

/// Returns a new in-memory [`StatefulKV`].
pub fn memory_state_kv_init() -> Rc<RefCell<dyn StatefulKV>> {
    Rc::new(RefCell::new(FakeKV::new()))
}

/// Creates an in-memory `Runtime` backed by a `state_kv`.
pub fn create_memory_runtime(
    state_kv: &Rc<RefCell<dyn StatefulKV>>,
) -> DefaultRuntime<DefaultMemEnvTypes> {
    let storage_builder = runtime_memory_storage_builder(state_kv);

    let template_store = DefaultMemTemplateStore::new();
    let account_store = DefaultMemAccountStore::new();
    let env = Env::<DefaultMemEnvTypes>::new(account_store, template_store);

    let config = Config::default();
    let imports = ("sm".to_string(), wasmer::Exports::new());

    DefaultRuntime::new(env, imports, Box::new(storage_builder), config, None)
}

/// Returns a function (wrapped inside [`Box`]) that initializes `Account`'s storage client.
pub fn runtime_memory_storage_builder(
    state_kv: &Rc<RefCell<dyn StatefulKV>>,
) -> Box<StorageBuilderFn> {
    let state_kv = Rc::clone(state_kv);

    let func =
        move |account_addr: &AccountAddr, state: &State, layout: &FixedLayout, _config: &Config| {
            let account_addr = account_addr.inner();
            let account_kv = AccountKVStore::new(account_addr.clone(), &state_kv);

            let mut storage = AccountStorage::new(layout.clone(), account_kv);
            storage.rewind(state);

            storage
        };

    Box::new(func)
}

/// Builds a raw `Deploy Template` transaction.
pub fn build_template(
    code_version: u32,
    name: &str,
    layout: FixedLayout,
    ctors: &[String],
    wasm: WasmFile,
) -> Vec<u8> {
    let code = CodeSection::new_fixed(wasm.into_bytes(), 0);
    let ctors = CtorsSection::new(ctors.to_vec());
    let data = DataSection::with_layout(Layout::Fixed(layout));
    let header = HeaderSection::new(code_version, name.to_string(), "".to_string());

    let template = TemplateBuilder::default()
        .with_code(code)
        .with_data(data)
        .with_ctors(ctors)
        .with_header(header)
        .build();

    template::encode(&template)
}

/// Builds a raw `Spawn Account` transaction.
pub fn build_spawn(template: &TemplateAddr, name: &str, ctor: &str, calldata: &[u8]) -> Vec<u8> {
    SpawnBuilder::new()
        .with_version(0)
        .with_template(template)
        .with_name(name)
        .with_ctor(ctor)
        .with_calldata(calldata)
        .build()
}

/// Builds a raw `Call App` transaction. (a.k.a a [`Transaction`]).
pub fn build_transaction(target: &AccountAddr, func: &str, calldata: &[u8]) -> Vec<u8> {
    TxBuilder::new()
        .with_version(0)
        .with_target(target)
        .with_func(func)
        .with_calldata(calldata)
        .build()
}
