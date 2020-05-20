use std::{cell::RefCell, collections::HashMap, ffi::c_void, path::Path, rc::Rc};

use crate::{
    buffer::BufferRef,
    ctx::SvmCtx,
    gas::{DefaultGasEstimator, MaybeGas},
    helpers::{self, DataWrapper},
    settings::AppSettings,
    storage::StorageBuilderFn,
    DefaultRuntime,
};
use svm_app::{
    memory::{DefaultMemAppStore, DefaultMemAppTemplateStore, DefaultMemoryEnv},
    testing::{AppTxBuilder, DeployAppTemplateBuilder, HostCtxBuilder, SpawnAppBuilder},
    types::{AppAddr, TemplateAddr, WasmValue},
};
use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;
use svm_layout::DataLayout;
use svm_storage::{
    app::{AppKVStore, AppStorage},
    kv::{FakeKV, StatefulKVStore},
};
use wasmer_runtime_core::{export::Export, import::ImportObject, Instance, Module};

/// Compiles a wasm program in text format (a.k.a WAST) into a `Module` (`wasmer`)
pub fn wasmer_compile(wasm: &str, gas_limit: MaybeGas) -> Module {
    let wasm = wabt::wat2wasm(&wasm).unwrap();

    let gas_metering = gas_limit.is_some();
    let gas_limit = gas_limit.unwrap_or(0);

    svm_compiler::compile_program(&wasm[..], gas_limit, gas_metering).unwrap()
}

/// Instantiate a `wasmer` instance
pub fn instantiate(import_object: &ImportObject, wasm: &str, gas_limit: MaybeGas) -> Instance {
    let module = wasmer_compile(wasm, gas_limit);
    module.instantiate(import_object).unwrap()
}

/// Mutably borrows the `AppStorage` of a living `App` instance.
pub fn instance_storage(instance: &Instance) -> &mut AppStorage {
    helpers::wasmer_data_app_storage(instance.context().data)
}

/// Mutably borrows the Buffer with id `buf_id` of a living `App` instance.
pub fn instance_buffer(instance: &Instance, buf_id: u32) -> Option<&mut BufferRef> {
    helpers::wasmer_data_buffer(instance.context().data, buf_id)
}

/// Returns a view of `wasmer` instance memory at `offset`...`offest + len - 1`
pub fn instance_memory_view(instance: &Instance, offset: u32, len: u32) -> Vec<u8> {
    let view = instance.context().memory(0).view();

    let start = offset as usize;
    let end = start + len as usize;

    view[start..end].iter().map(|cell| cell.get()).collect()
}

/// Copies input slice `bytes` into `wasmer` instance memory starting at offset `offset`.
pub fn instance_memory_init(instance: &Instance, offset: u32, bytes: &[u8]) {
    let view = instance.context().memory(0).view();

    let start = offset as usize;
    let end = start + bytes.len() as usize;
    let cells = &view[start..end];

    for (cell, byte) in cells.iter().zip(bytes.iter()) {
        cell.set(*byte);
    }
}

/// Returns a `state creator` to be used by wasmer `ImportObject::new_with_data` initializer.
pub fn app_memory_state_creator(
    app_addr: &Address,
    state: &State,
    host: DataWrapper<*mut c_void>,
    host_ctx: DataWrapper<*const c_void>,
    gas_limit: MaybeGas,
    layout: &DataLayout,
) -> (*mut c_void, fn(*mut c_void)) {
    let raw_kv = memory_kv_store2_init();
    let app_kv = AppKVStore::new(app_addr.clone(), &raw_kv);
    let storage = AppStorage::new(layout.clone(), app_kv);

    let ctx = SvmCtx::new(host, host_ctx, gas_limit, storage);
    let ctx: *mut SvmCtx = Box::into_raw(Box::new(ctx));

    let data: *mut c_void = ctx as *const _ as _;
    let dtor: fn(*mut c_void) = |_| {};

    (data, dtor)
}

/// Initializes a new in-memory key-value store.
pub fn memory_kv_store_init() -> Rc<RefCell<dyn StatefulKVStore>> {
    Rc::new(RefCell::new(FakeKV::new()))
}

/// Creates an in-memory `Runtime` backed by key-value, raw pointer to host and host vmcalls (`imports`)
pub fn create_memory_runtime(
    host: *mut c_void,
    raw_kv: &Rc<RefCell<dyn StatefulKVStore>>,
    imports: Vec<(String, String, Export)>,
) -> DefaultRuntime<DefaultMemoryEnv, DefaultGasEstimator> {
    let storage_builder = runtime_memory_storage2_builder(raw_kv);

    let env = runtime_memory_env_builder();
    let kv_path = Path::new("mem");

    DefaultRuntime::new(host, env, &kv_path, imports, Box::new(storage_builder))
}

/// Returns a function (wrapped inside `Box`) that initializes an App's storage client.
pub fn runtime_memory_storage_builder(
    raw_kv: &Rc<RefCell<dyn StatefulKVStore>>,
) -> Box<StorageBuilderFn> {
    let raw_kv = Rc::clone(raw_kv);

    let func = move |app_addr: &AppAddr, _state: &State, settings: &AppSettings| {
        let layout = settings.layout.clone();

        let app_addr = app_addr.inner();
        let app_kv = AppKVStore::new(app_addr.clone(), &raw_kv);

        AppStorage::new(layout, app_kv)
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
pub fn build_template(
    version: u32,
    name: &str,
    data: DataLayout,
    wasm: &str,
    is_wast: bool,
) -> Vec<u8> {
    let code = if is_wast {
        wabt::wat2wasm(wasm).unwrap()
    } else {
        wasm.as_bytes().to_vec()
    };

    DeployAppTemplateBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_code(code.as_slice())
        .with_data(&data)
        .build()
}

/// Synthesizes a raw spaw-app transaction.
pub fn build_app(
    version: u32,
    template: &TemplateAddr,
    ctor_idx: u16,
    ctor_buf: &Vec<u8>,
    ctor_args: &Vec<WasmValue>,
) -> Vec<u8> {
    SpawnAppBuilder::new()
        .with_version(version)
        .with_template(template)
        .with_ctor_index(ctor_idx)
        .with_ctor_buf(ctor_buf)
        .with_ctor_args(ctor_args)
        .build()
}

/// Synthesizes a raw exec-app transaction.
pub fn build_app_tx(
    version: u32,
    app_addr: &AppAddr,
    func_idx: u16,
    func_buf: &Vec<u8>,
    func_args: &Vec<WasmValue>,
) -> Vec<u8> {
    AppTxBuilder::new()
        .with_version(version)
        .with_app(app_addr)
        .with_func_index(func_idx)
        .with_func_buf(func_buf)
        .with_func_args(func_args)
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
