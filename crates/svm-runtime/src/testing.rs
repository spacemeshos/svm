use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::rc::Rc;

use crate::ctx::SvmCtx;
use crate::ctx_data_wrapper::SvmCtxDataWrapper;
use crate::opts::Opts;
use crate::runtime::Runtime;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;
use svm_storage::{default::DefaultPageCache, memory::MemContractPages, ContractStorage};

use wasmer_runtime_core::{import::ImportObject, Instance, Module};

use svm_contract::build::{WireContractBuilder, WireTxBuilder};
use svm_contract::memory::{MemContractStore, MemoryEnv};
use svm_contract::wasm::WasmArgValue;

pub fn wasmer_compile(wasm: &str) -> Module {
    let wasm = wabt::wat2wasm(&wasm).unwrap();
    svm_compiler::compile_program(&wasm).unwrap()
}

pub fn instantiate(import_object: &ImportObject, wasm: &str) -> Instance {
    let module = wasmer_compile(wasm);
    let instance = module.instantiate(import_object).unwrap();
    instance
}

pub fn instance_memory_view(instance: &Instance, offset: usize, len: usize) -> Vec<u8> {
    let view = instance.context().memory(0).view();

    view[offset..offset + len]
        .iter()
        .map(|cell| cell.get())
        .collect()
}

pub fn instance_memory_init(instance: &mut Instance, offset: usize, bytes: &[u8]) {
    let view = instance.context().memory(0).view();
    let cells = &view[offset..(offset as usize + bytes.len())];

    for (cell, byte) in cells.iter().zip(bytes.iter()) {
        cell.set(*byte);
    }
}

pub fn contract_memory_state_creator(
    addr: u32,
    state: u32,
    wrapped_node_data: SvmCtxDataWrapper,
    pages_count: u32,
) -> (*mut c_void, fn(*mut c_void)) {
    let addr = Address::from(addr);
    let state = State::from(state);
    let kv = memory_kv_store_init();

    let storage = svm_storage::testing::contract_storage_open(&addr, &state, &kv, pages_count);

    let ctx = SvmCtx::new(wrapped_node_data, storage);
    let ctx: *mut SvmCtx = Box::into_raw(Box::new(ctx));

    let data: *mut c_void = ctx as *const _ as _;
    let dtor: fn(*mut c_void) = |_| {};

    (data, dtor)
}

pub fn memory_kv_store_init() -> Rc<RefCell<MemKVStore>> {
    Rc::new(RefCell::new(MemKVStore::new()))
}

pub fn runtime_memory_storage_builder(
    kv: &Rc<RefCell<MemKVStore>>,
) -> Box<dyn Fn(Address, State, &Opts) -> ContractStorage> {
    let kv = Rc::clone(kv);

    let builder = move |addr: Address, state: State, opts: &Opts| {
        svm_storage::testing::contract_storage_open(&addr, &state, &kv, opts.max_pages as u32)
    };

    Box::new(builder)
}

pub fn runtime_memory_env_builder(kv: &Rc<RefCell<MemKVStore>>) -> Box<dyn Fn(&str) -> MemoryEnv> {
    let kv = Rc::clone(kv);

    let builder = move |_path: &str| {
        let store = MemContractStore::new();
        MemoryEnv::new(store)
    };

    Box::new(builder)
}

pub fn create_memory_runtime(kv: &Rc<RefCell<MemKVStore>>) -> Runtime<MemoryEnv> {
    let storage_builder = runtime_memory_storage_builder(kv);
    let env_builder = runtime_memory_env_builder(kv);

    Runtime::new(Box::new(env_builder), Box::new(storage_builder))
}

pub fn build_raw_contract(version: u32, name: &str, author: u32, wasm: &str) -> Vec<u8> {
    let wasm = wabt::wat2wasm(wasm).unwrap();

    WireContractBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_author(Address::from(author))
        .with_code(wasm.as_slice())
        .build()
}

pub fn build_raw_transaction(
    version: u32,
    contract_addr: &Address,
    sender_addr: u32,
    func_name: &str,
    func_args: &[WasmArgValue],
) -> Vec<u8> {
    WireTxBuilder::new()
        .with_version(version)
        .with_contract(contract_addr.clone())
        .with_sender(Address::from(sender_addr))
        .with_func_name(func_name)
        .with_func_args(func_args)
        .build()
}
