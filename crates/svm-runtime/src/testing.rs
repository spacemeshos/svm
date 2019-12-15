use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::rc::Rc;

use crate::ctx::SvmCtx;
use crate::ctx_data_wrapper::SvmCtxDataWrapper;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;
use svm_storage::{default::DefaultPageCache, memory::MemContractPages, ContractStorage};

use wasmer_runtime_core::{import::ImportObject, Instance, Module};

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

    let kv = Rc::new(RefCell::new(MemKVStore::new()));
    let pages = MemContractPages::new(addr, kv, state, pages_count);
    let cache = DefaultPageCache::new(pages, pages_count as usize);
    let storage = ContractStorage::new(Box::new(cache));

    let ctx = SvmCtx::new(wrapped_node_data, storage);
    let ctx = Box::new(ctx);
    let ctx: *mut SvmCtx = Box::into_raw(ctx);

    let data: *mut c_void = ctx as *const _ as _;
    let dtor: fn(*mut c_void) = |_| {};

    (data, dtor)
}
