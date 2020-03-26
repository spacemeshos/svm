use std::{cell::RefCell, collections::HashMap, ffi::c_void, path::Path, rc::Rc};

use crate::{
    buffer::BufferRef,
    ctx::SvmCtx,
    gas::DefaultGasEstimator,
    helpers::{self, DataWrapper},
    register::Register,
    settings::AppSettings,
    storage::StorageBuilderFn,
    DefaultRuntime,
};

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;
use svm_storage::AppStorage;

use svm_app::{
    memory::{DefaultMemAppStore, DefaultMemAppTemplateStore, DefaultMemoryEnv},
    testing::{AppTxBuilder, DeployAppTemplateBuilder, HostCtxBuilder, SpawnAppBuilder},
    types::{AppAddr, TemplateAddr, WasmValue},
};

use wasmer_runtime_core::{export::Export, import::ImportObject, Instance, Module};

/// Compiles a wasm program in text format (a.k.a WAST) into a `Module` (`wasmer`)
pub fn wasmer_compile(wasm: &str) -> Module {
    let wasm = wabt::wat2wasm(&wasm).unwrap();
    svm_compiler::compile_program(&wasm).unwrap()
}

/// Instantiate a `wasmer` instance
pub fn instantiate(import_object: &ImportObject, wasm: &str) -> Instance {
    let module = wasmer_compile(wasm);
    module.instantiate(import_object).unwrap()
}

/// Mutably borrows `SVM` register `reg_bits:reg_idx`
pub fn instance_register(instance: &Instance, reg_bits: u32, reg_idx: u32) -> &mut Register {
    helpers::wasmer_data_reg(instance.context().data, reg_bits, reg_idx)
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
    _app_addr: &Address,
    state: &State,
    host: DataWrapper<*mut c_void>,
    host_ctx: DataWrapper<*const c_void>,
    page_count: u16,
) -> (*mut c_void, fn(*mut c_void)) {
    let kv = memory_kv_store_init();

    let storage = svm_storage::testing::app_storage_open(state, &kv, page_count);

    let ctx = SvmCtx::new(host, host_ctx, storage);
    let ctx: *mut SvmCtx = Box::into_raw(Box::new(ctx));

    let data: *mut c_void = ctx as *const _ as _;
    let dtor: fn(*mut c_void) = |_| {};

    (data, dtor)
}

/// Initializes a new `MemKVStore`
pub fn memory_kv_store_init() -> Rc<RefCell<MemKVStore>> {
    Rc::new(RefCell::new(MemKVStore::new()))
}

/// Creates an in-memory `Runtime` backed by key-value, raw pointer to host and host vmcalls (`imports`)
pub fn create_memory_runtime(
    host: *mut c_void,
    kv: &Rc<RefCell<MemKVStore>>,
    imports: Vec<(String, String, Export)>,
) -> DefaultRuntime<DefaultMemoryEnv, DefaultGasEstimator> {
    let storage_builder = runtime_memory_storage_builder(kv);

    let env = runtime_memory_env_builder();
    let kv_path = Path::new("mem");

    DefaultRuntime::new(host, env, &kv_path, imports, Box::new(storage_builder))
}

/// Creates an app storage builder function backed by key-value store `kv`.
pub fn runtime_memory_storage_builder(kv: &Rc<RefCell<MemKVStore>>) -> Box<StorageBuilderFn> {
    let kv = Rc::clone(kv);

    let func = move |_addr: &AppAddr, state: &State, settings: &AppSettings| {
        svm_storage::testing::app_storage_open(state, &kv, settings.page_count)
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
pub fn build_template(version: u32, name: &str, page_count: u16, wasm: &str) -> Vec<u8> {
    let code = wabt::wat2wasm(wasm).unwrap();

    DeployAppTemplateBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_page_count(page_count)
        .with_code(code.as_slice())
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
