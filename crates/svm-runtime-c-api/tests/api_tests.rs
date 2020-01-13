extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;
use svm_runtime_c_api::{svm_import_t, svm_value_type, testing, testing::host_ctx};

use std::collections::HashMap;
use std::ffi::c_void;

use svm_app::types::WasmArgValue;
use svm_common::{Address, State};
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

fn deploy_template_bytes(name: &str, author: u32, pages_count: u16, wasm: &str) -> (Vec<u8>, u64) {
    let bytes = svm_runtime::testing::build_template(0, name, author, pages_count, wasm);
    let bytes_len = bytes.len() as u64;

    (bytes, bytes_len)
}

fn spawn_app_bytes(creator: u32, template_addr: *const c_void) -> (Vec<u8>, u64) {
    let template_addr: &Address = unsafe { svm_common::from_raw::<Address>(template_addr) };

    let bytes = svm_runtime::testing::build_app(0, template_addr, creator);
    let bytes_len = bytes.len() as u64;

    (bytes, bytes_len)
}

fn exec_app_bytes(
    sender_addr: u32,
    app_addr: *const c_void,
    func_name: &str,
    func_args: &[WasmArgValue],
) -> (Vec<u8>, u64) {
    let app_addr: &Address = unsafe { svm_common::from_raw::<Address>(app_addr) };

    let bytes = svm_runtime::testing::build_app_tx(0, app_addr, sender_addr, func_name, func_args);
    let bytes_len = bytes.len() as u64;

    (bytes, bytes_len)
}

fn exec_app_args() -> (u32, i64, Vec<WasmArgValue>, State) {
    let sender = 0x50_60_70_80;
    let mul_by = 3;
    let args = vec![WasmArgValue::I64(mul_by)];
    let state = State::empty();

    (sender, mul_by as i64, args, state)
}

#[test]
fn runtime_ffi_exec_app() {
    unsafe {
        do_exec_app();
    }
}

unsafe fn do_exec_app() {
    // 1) init runtime
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

    // 2) deploy app-template
    let author = 0x10_20_30_40;
    let code = include_str!("wasm/update-balance.wast");
    let pages_count = 10;
    let (bytes, bytes_len) = deploy_template_bytes("MyTemplate #1", author, pages_count, code);
    let mut template = std::ptr::null_mut();

    let res = api::svm_deploy_template(&mut template, runtime, bytes.as_ptr() as _, bytes_len);
    assert_eq!(true, res.as_bool());

    // 3) spawn app
    let mut app_addr = std::ptr::null_mut();
    let creator = 0x20_30_40_50;
    let (bytes, bytes_len) = spawn_app_bytes(creator, template as _);
    let res = api::svm_spawn_app(&mut app_addr, runtime, bytes.as_ptr() as _, bytes_len);
    assert_eq!(true, res.as_bool());

    // 3) execute app
    let (sender, mul_by, args, state) = exec_app_args();
    let (bytes, bytes_len) = exec_app_bytes(sender, app_addr, "run", &args);

    // 3.1) parse bytes into in-memory `AppTransaction`
    let mut app_tx = std::ptr::null_mut();
    let res = api::svm_parse_exec_app(&mut app_tx, runtime, bytes.as_ptr() as _, bytes_len);
    assert_eq!(true, res.as_bool());

    // 3.2) execute the app-transaction
    // initialize `address=0x10_20_30` with balance=100
    host.set_balance(&Address::from(0x10_20_30), 100);
    assert_eq!(100, host.get_balance(&Address::from(0x10_20_30)).unwrap());

    let delta = 50;
    let delta_vec = vec![0, 0, 0, 0, 0, 0, 0, delta];
    // we set field index `2` with a value called `delta` (one byte).
    let mut host_ctx = Vec::new();
    host_ctx::write_version(&mut host_ctx, 0);
    host_ctx::write_field_count(&mut host_ctx, 1);
    host_ctx::write_field(&mut host_ctx, 2, delta_vec);

    let mut receipt = std::ptr::null_mut();
    let mut receipt_length = 0;
    let res = api::svm_exec_app(
        &mut receipt,
        &mut receipt_length,
        runtime,
        app_tx,
        svm_common::into_raw(state),
        host_ctx.as_ptr() as _,
        host_ctx.len() as _,
    );
    assert_eq!(true, res.as_bool());

    assert_eq!(
        100 * mul_by + (delta as i64),
        host.get_balance(&Address::from(0x10_20_30)).unwrap()
    );
}
