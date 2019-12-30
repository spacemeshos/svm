extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;
use svm_runtime_c_api::{svm_import_t, svm_value_type, testing};

use std::collections::HashMap;
use std::ffi::c_void;

use svm_common::{Address, State};
use svm_contract::{
    build::{WireContractBuilder, WireTxBuilder},
    wasm::WasmArgValue,
};
use svm_runtime::register::SvmReg;

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

unsafe fn extract_host<'a>(raw_ctx: *mut c_void) -> &'a mut Host {
    let host = api::svm_instance_context_host_get(raw_ctx);
    svm_common::from_raw_mut::<Host>(host)
}

unsafe fn extract_reg<'a>(raw_ctx: *mut c_void, reg_bits: i32, reg_idx: i32) -> &'a mut SvmReg {
    use wasmer_runtime_core::vm::Ctx as WasmerCtx;

    let ctx = svm_common::from_raw_mut::<WasmerCtx>(raw_ctx);

    svm_runtime::helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx)
}

unsafe extern "C" fn get_balance(ctx: *mut c_void, reg_bits: i32, reg_idx: i32) -> i64 {
    let host = extract_host(ctx);
    let reg = extract_reg(ctx, reg_bits, reg_idx);

    let addr = Address::from(reg.as_ptr());
    host.get_balance(&addr).unwrap_or(0)
}

unsafe extern "C" fn set_balance(ctx: *mut c_void, value: i64, reg_bits: i32, reg_idx: i32) {
    let host = extract_host(ctx);
    let reg = extract_reg(ctx, reg_bits, reg_idx);

    let addr = Address::from(reg.as_ptr());
    host.set_balance(&addr, value);
}

unsafe fn create_imports() -> (Vec<*const svm_import_t>, u32) {
    let get_balance_import = testing::import_func_create(
        "env",
        "get_balance",
        get_balance as _,
        vec![svm_value_type::SVM_I32, svm_value_type::SVM_I32],
        vec![svm_value_type::SVM_I64],
    );

    let set_balance_import = testing::import_func_create(
        "env",
        "set_balance",
        set_balance as _,
        vec![
            svm_value_type::SVM_I64,
            svm_value_type::SVM_I32,
            svm_value_type::SVM_I32,
        ],
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
    let addr: &Address = unsafe { svm_common::from_raw::<Address>(addr) };
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

fn transaction_exec_args() -> (u32, i64, Vec<WasmArgValue>, State, i32) {
    let sender = 0x50_60_70_80;
    let mul_by = 3;
    let args = vec![WasmArgValue::I64(mul_by)];
    let state = State::empty();
    let pages_count = 10;

    (sender, mul_by as i64, args, state, pages_count)
}

#[test]
fn runtime_c_transaction_exec() {
    unsafe {
        do_transaction_exec();
    }
}

unsafe fn do_transaction_exec() {
    let mut host = Host::new();
    let mut kv = std::ptr::null_mut();
    let mut runtime = std::ptr::null_mut();
    let (imports, imports_len) = create_imports();

    testing::svm_memory_kv_create(&mut kv);

    let res = testing::svm_memory_runtime_create(
        &mut runtime,
        kv,
        host.as_mut_ptr(),
        imports.as_ptr(),
        imports_len,
    );
    assert_eq!(true, res.as_bool());

    let author = 0x10_20_30_40;
    let code = include_str!("wasm/mul_balance.wast");
    let (bytes, bytes_len) = deploy_contract_bytes("Contract #1", code, author);
    let mut contract = std::ptr::null_mut();

    let res = api::svm_contract_build(&mut contract, runtime, bytes.as_ptr() as _, bytes_len);
    assert_eq!(true, res.as_bool());

    let addr = api::svm_contract_derive_address(runtime, contract);
    let res = api::svm_contract_deploy(runtime, contract, addr);
    assert_eq!(true, res.as_bool());

    let (sender, mul_by, args, state, pages_count) = transaction_exec_args();
    let (bytes, bytes_len) = transaction_exec_bytes(addr, sender, "run", &args);

    let mut tx = std::ptr::null_mut();
    let res = api::svm_transaction_build(&mut tx, runtime, bytes.as_ptr() as _, bytes_len);
    assert_eq!(true, res.as_bool());

    // initialize `address=0x10_20_30` with balance=100
    host.set_balance(&Address::from(0x10_20_30), 100);
    assert_eq!(100, host.get_balance(&Address::from(0x10_20_30)).unwrap());

    let mut receipt = std::ptr::null_mut();
    let res = api::svm_transaction_exec(
        &mut receipt,
        runtime,
        tx,
        svm_common::into_raw(state),
        pages_count,
    );
    assert_eq!(true, res.as_bool());

    assert_eq!(
        100 * mul_by as i64,
        host.get_balance(&Address::from(0x10_20_30)).unwrap()
    );
}
