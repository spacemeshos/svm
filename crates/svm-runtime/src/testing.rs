use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;

use crate::{
    buffer::BufferRef, ctx::SvmCtx, helpers, helpers::DataWrapper, register::Register,
    settings::AppSettings, traits::StorageBuilderFn, DefaultRuntime,
};

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;
use svm_storage::AppStorage;

use svm_app::{
    memory::{JsonMemAppStore, JsonMemAppTemplateStore, JsonMemoryEnv},
    testing::{AppBuilder, AppTemplateBuilder, AppTxBuilder, HostCtxBuilder},
    types::WasmValue,
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
    let instance = module.instantiate(import_object).unwrap();
    instance
}

/// Mutably borrows `SVM` register `reg_bits:reg_idx`
pub fn instance_register(instance: &Instance, reg_bits: i32, reg_idx: i32) -> &mut Register {
    helpers::wasmer_data_reg(instance.context().data, reg_bits, reg_idx)
}

/// Mutably borrows the `AppStorage` of a living `App` instance.
pub fn instance_storage(instance: &Instance) -> &mut AppStorage {
    helpers::wasmer_data_app_storage(instance.context().data)
}

pub fn instance_buffer(instance: &Instance, buf_id: i32) -> Option<&mut BufferRef> {
    helpers::wasmer_data_buffer(instance.context().data, buf_id)
}

/// Returns a view of `wasmer` instance memory at `offset`...`offest + len - 1`
pub fn instance_memory_view(instance: &Instance, offset: i32, len: i32) -> Vec<u8> {
    let view = instance.context().memory(0).view();

    let start = offset as usize;
    let end = start + len as usize;

    view[start..end].iter().map(|cell| cell.get()).collect()
}

/// Copies input slice `bytes` into `wasmer` instance memory starting at offset `offset`.
pub fn instance_memory_init(instance: &Instance, offset: i32, bytes: &[u8]) {
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
    page_count: u16,
) -> (*mut c_void, fn(*mut c_void)) {
    let kv = memory_kv_store_init();

    let storage = svm_storage::testing::app_storage_open(app_addr, state, &kv, page_count);

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
) -> DefaultRuntime<JsonMemoryEnv> {
    let storage_builder = runtime_memory_storage_builder(kv);

    let env = runtime_memory_env_builder();

    DefaultRuntime::new(host, env, imports, Box::new(storage_builder))
}

/// Creates an app storage builder function backed by key-value store `kv`.
pub fn runtime_memory_storage_builder(kv: &Rc<RefCell<MemKVStore>>) -> Box<StorageBuilderFn> {
    let kv = Rc::clone(kv);

    let func = move |addr: &Address, state: &State, settings: &AppSettings| {
        svm_storage::testing::app_storage_open(addr, state, &kv, settings.page_count)
    };

    Box::new(func)
}

/// Creates a new in-memory runtime environment.
pub fn runtime_memory_env_builder() -> JsonMemoryEnv {
    let template_store = JsonMemAppTemplateStore::new();
    let app_store = JsonMemAppStore::new();

    JsonMemoryEnv::new(app_store, template_store)
}

/// Synthesizes a raw deploy-template transaction.
pub fn build_template(version: u32, name: &str, page_count: u16, wasm: &str) -> Vec<u8> {
    let code = wabt::wat2wasm(wasm).unwrap();

    AppTemplateBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_page_count(page_count)
        .with_code(code.as_slice())
        .build()
}

/// Synthesizes a raw spaw-app transaction.
pub fn build_app(
    version: u32,
    template: &Address,
    ctor_buf: &Vec<Vec<u8>>,
    ctor_args: &Vec<WasmValue>,
) -> Vec<u8> {
    AppBuilder::new()
        .with_version(version)
        .with_template(template)
        .with_ctor_buf(ctor_buf)
        .with_ctor_args(ctor_args)
        .build()
}

/// Synthesizes a raw exec-app transaction.
pub fn build_app_tx(
    version: u32,
    app_addr: &Address,
    func_name: &str,
    func_buf: &Vec<Vec<u8>>,
    func_args: &Vec<WasmValue>,
) -> Vec<u8> {
    AppTxBuilder::new()
        .with_version(version)
        .with_app(app_addr)
        .with_func_name(func_name)
        .with_func_buf(func_buf)
        .with_func_args(func_args)
        .build()
}

pub fn build_host_ctx(version: u32, fields: HashMap<i32, Vec<u8>>) -> Vec<u8> {
    let mut builder = HostCtxBuilder::new().with_version(version);

    for (idx, value) in fields.iter() {
        builder = builder.with_raw_field(*idx, &value[..]);
    }

    builder.build()
}
