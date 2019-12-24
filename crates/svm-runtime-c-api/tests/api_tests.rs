#![allow(unused)]

extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;
use svm_runtime_c_api::{helpers, testing};

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;

use wasmer_runtime_c_api::{
    export::{wasmer_import_export_kind, wasmer_import_export_value},
    import::{wasmer_import_func_new, wasmer_import_func_t, wasmer_import_t},
    instance::wasmer_instance_context_t,
    value::wasmer_value_tag,
    wasmer_byte_array, wasmer_result_t,
};

use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
    vm::{Ctx, Func},
};

use svm_common::{Address, State};
use svm_contract::{
    build::{WireContractBuilder, WireTxBuilder},
    wasm::WasmArgValue,
};

struct Host {
    balance: HashMap<Address, i64>,
}

impl Host {
    fn new() -> Self {
        Self {
            balance: HashMap::new(),
        }
    }

    fn get_balance(&self, addr: &Address) -> Option<i64> {
        self.balance.get(addr).copied()
    }

    fn set_balance(&mut self, addr: &Address, balance: i64) {
        self.balance.insert(addr.clone(), balance);
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut Self as _
    }
}

extern "C" fn vmcall_get_balance(
    ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
) -> i64 {
    0
}

extern "C" fn vmcall_set_balance(
    ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
    balance: i64,
) {
    //
}

macro_rules! raw_kv {
    ($kv:ident) => {{
        use std::cell::RefCell;
        use std::rc::Rc;

        &$kv as *const Rc<RefCell<_>> as _
    }};
}

macro_rules! raw_func {
    ($func:ident) => {{
        $func as *mut _
    }};
}

macro_rules! raw_imports {
    ($imports:ident) => {{
        $imports.as_mut_ptr() as *mut _
    }};
}

#[test]
fn sanity() {
    unsafe {
        unsafe_sanity();
    }
}

unsafe fn create_imports() -> (Vec<wasmer_import_t>, u32) {
    let get_balance_import = testing::wasmer_import_func_create(
        "env",
        "get_balance",
        raw_func!(vmcall_get_balance),
        vec![Type::I32, Type::I32],
        vec![Type::I64],
    );

    let set_balance_import = testing::wasmer_import_func_create(
        "env",
        "set_balance",
        raw_func!(vmcall_set_balance),
        vec![Type::I32, Type::I32, Type::I64],
        vec![],
    );

    let imports = vec![get_balance_import, set_balance_import];
    let imports_len = imports.len() as u32;

    (imports, imports_len)
}

fn deploy_contract_bytes(name: &str, wasm: &str, author: u32) -> (Vec<u8>, u64) {
    let wasm = wabt::wat2wasm(wasm).unwrap();
    let author = Address::from(author);

    let bytes = WireContractBuilder::new()
        .with_version(0)
        .with_author(author)
        .with_code(wasm.as_slice())
        .with_name(name)
        .build();

    let bytes_len = bytes.len() as u64;
    (bytes, bytes_len)
}

fn transaction_exec_bytes(
    addr: *const c_void,
    sender_addr: u32,
    func_name: &str,
    func_args: &[WasmArgValue],
) -> (Vec<u8>, u64) {
    let addr: &Address = unsafe { helpers::from_raw::<Address>(addr) };
    let sender_addr = Address::from(sender_addr);

    let bytes = WireTxBuilder::new()
        .with_version(0)
        .with_contract(addr.clone())
        .with_sender(sender_addr)
        .with_func_name(func_name)
        .with_func_args(func_args)
        .build();

    let bytes_len = bytes.len() as u64;
    (bytes, bytes_len)
}

unsafe fn unsafe_sanity() {
    let mut host = Host::new();
    let mut runtime = std::ptr::null_mut();
    let (mut imports, imports_len) = create_imports();

    let kv = svm_runtime::testing::memory_kv_store_init();

    let _res = testing::svm_memory_runtime_create(
        &mut runtime,
        raw_kv!(kv),
        host.as_mut_ptr(),
        raw_imports!(imports),
        imports_len,
    );
    // TODO: assert that `res` is `wasmer_result_t::WASMER_OK`

    let author = 0x10_20_30_40;
    let (bytes, bytes_len) =
        deploy_contract_bytes("Contract #1", include_str!("wasm/mul_balance.wast"), author);
    let mut contract = std::ptr::null_mut();

    let _res = api::svm_contract_build(&mut contract, runtime, bytes.as_ptr() as _, bytes_len);
    // TODO: assert that `res` is `wasmer_result_t::WASMER_OK`

    let addr = api::svm_contract_derive_address(runtime, contract);
    let _res = api::svm_contract_deploy(runtime, contract, addr);
    // TODO: assert that `res` is `wasmer_result_t::WASMER_OK`

    let mut tx = std::ptr::null_mut();
    let sender = 0x50_60_70_80;
    let args = [];
    let state = State::empty();
    let pages_count = 10;

    let (bytes, bytes_len) = transaction_exec_bytes(addr, sender, "run", &args);
    let _res = api::svm_transaction_build(&mut tx, runtime, bytes.as_ptr() as _, bytes_len);
    // TODO: assert that `res` is `wasmer_result_t::WASMER_OK`

    let mut receipt = std::ptr::null_mut();
    let _res = api::svm_transaction_exec(
        &mut receipt,
        runtime,
        tx,
        helpers::into_raw(state),
        pages_count,
    );
    // TODO: assert that `res` is `wasmer_result_t::WASMER_OK`

    // dbg!(kv.borrow().keys());
}
