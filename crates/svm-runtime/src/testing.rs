use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::path::Path;
use std::rc::Rc;

use crate::env::memory::{DefaultMemAppStore, DefaultMemAppTemplateStore, DefaultMemoryEnv};
use crate::{gas::DefaultGasEstimator, storage::StorageBuilderFn};
use crate::{Config, Context, DefaultRuntime, Import};

use svm_codec::api::builder::{
    AppTxBuilder, DeployAppTemplateBuilder, HostCtxBuilder, SpawnAppBuilder,
};
use svm_layout::DataLayout;
use svm_storage::{
    app::{AppKVStore, AppStorage},
    kv::{FakeKV, StatefulKV},
};
use svm_types::{gas::MaybeGas, receipt::Log, Address, AppAddr, State, TemplateAddr, WasmValue};

use wasmer::{Export, ImportObject, Instance, Memory, MemoryType, Module, Pages, Store};

pub enum WasmFile<'a> {
    Text(&'a str),

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

pub fn wasmer_store() -> Store {
    svm_compiler::new_store()
}

pub fn wasmer_memory(store: &Store) -> Memory {
    let min = Pages(1);
    let max = None;
    let shared = false;
    let ty = MemoryType::new(min, max, shared);

    Memory::new(store, ty).expect("Memory allocation has failed.")
}

/// Compiles a wasm program in text format (a.k.a WAST) into a `Module` (`wasmer`)
pub fn wasmer_compile(store: &Store, wasm_file: WasmFile, gas_limit: MaybeGas) -> Module {
    let wasm = wasm_file.into_bytes();

    let gas_metering = gas_limit.is_some();
    let gas_limit = gas_limit.unwrap_or(0);

    svm_compiler::compile(store, &wasm, gas_limit, gas_metering).unwrap()
}

/// Instantiate a `wasmer` instance
pub fn wasmer_instantiate(
    store: &Store,
    import_object: &ImportObject,
    wasm_file: WasmFile,
    gas_limit: MaybeGas,
) -> Instance {
    let module = wasmer_compile(store, wasm_file, gas_limit);

    Instance::new(&module, import_object).unwrap()
}

pub fn blank_storage(app_addr: &Address, layout: &DataLayout) -> AppStorage {
    let state_kv = memory_state_kv_init();
    let app_kv = AppKVStore::new(app_addr.clone(), &state_kv);

    AppStorage::new(layout.clone(), app_kv)
}

/// Returns a new in-memory stateful-kv.
/// It should be used for managing apps' storage.
pub fn memory_state_kv_init() -> Rc<RefCell<dyn StatefulKV>> {
    Rc::new(RefCell::new(FakeKV::new()))
}

/// Creates an in-memory `Runtime` backed by key-value, raw pointer to host and host vmcalls (`imports`)
pub fn create_memory_runtime(
    host: *mut c_void,
    state_kv: &Rc<RefCell<dyn StatefulKV>>,
    imports: Vec<Import>,
) -> DefaultRuntime<DefaultMemoryEnv, DefaultGasEstimator> {
    let storage_builder = runtime_memory_storage_builder(state_kv);

    let env = runtime_memory_env_builder();
    let kv_path = Path::new("mem");

    DefaultRuntime::new(host, env, &kv_path, imports, Box::new(storage_builder))
}

/// Returns a function (wrapped inside `Box`) that initializes an App's storage client.
pub fn runtime_memory_storage_builder(
    state_kv: &Rc<RefCell<dyn StatefulKV>>,
) -> Box<StorageBuilderFn> {
    let state_kv = Rc::clone(state_kv);

    let func = move |app_addr: &AppAddr, state: &State, layout: &DataLayout, _config: &Config| {
        let app_addr = app_addr.inner();
        let app_kv = AppKVStore::new(app_addr.clone(), &state_kv);

        let mut storage = AppStorage::new(layout.clone(), app_kv);
        storage.rewind(state);

        storage
    };

    Box::new(func)
}

/// Creates a new in-memory runtime environment.
pub fn runtime_memory_env_builder() -> DefaultMemoryEnv {
    let template_store = DefaultMemAppTemplateStore::new();
    let app_store = DefaultMemAppStore::new();

    DefaultMemoryEnv::new(app_store, template_store)
}

/// Synthesizes a raw deploy-template transaction.
pub fn build_template(version: u32, name: &str, data: DataLayout, wasm: WasmFile) -> Vec<u8> {
    let wasm = wasm.into_bytes();

    DeployAppTemplateBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_code(&wasm)
        .with_data(&data)
        .build()
}

/// Synthesizes a raw spaw-app transaction.
pub fn build_app(
    version: u32,
    template: &TemplateAddr,
    name: &str,
    ctor: &str,
    calldata: &Vec<u8>,
) -> Vec<u8> {
    SpawnAppBuilder::new()
        .with_version(version)
        .with_template(template)
        .with_name(name)
        .with_ctor(ctor)
        .with_calldata(calldata)
        .build()
}

/// Synthesizes a raw exec-app transaction.
pub fn build_app_tx(version: u32, app_addr: &AppAddr, func: &str, calldata: &Vec<u8>) -> Vec<u8> {
    AppTxBuilder::new()
        .with_version(version)
        .with_app(app_addr)
        .with_func(func)
        .with_calldata(calldata)
        .build()
}

/// Encodes a raw `HostCtx` and returns it as `Vec<u8>`.
pub fn build_host_ctx(version: u32, fields: HashMap<u32, Vec<u8>>) -> Vec<u8> {
    let mut builder = HostCtxBuilder::new().with_version(version);

    for (idx, value) in fields.iter() {
        builder = builder.with_raw_field(*idx, &value[..]);
    }

    builder.build()
}
