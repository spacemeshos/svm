extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;
use svm_runtime_c_api::{svm_import_t, svm_value_type, testing};

use maplit::hashmap;

use std::collections::HashMap;
use std::ffi::c_void;

use svm_app::types::WasmValue;
use svm_common::{Address, State};
use svm_runtime::register::Register;

#[derive(Debug)]
struct Host {
    balance: HashMap<Address, i128>,
}

impl Host {
    fn new() -> Self {
        Self {
            balance: HashMap::new(),
        }
    }

    fn set_balance(&mut self, addr: &Address, init: i128) {
        self.balance.insert(addr.clone(), init);
    }

    fn get_balance(&self, addr: &Address) -> Option<i128> {
        self.balance.get(addr).copied()
    }

    fn inc_balance(&mut self, addr: &Address, addition: i64) {
        let balance = self.get_balance(addr).unwrap_or(0);

        let new_balance = balance + addition as i128;

        self.balance.insert(addr.clone(), new_balance);
    }

    fn mul_balance(&mut self, addr: &Address, mul_by: i64) {
        let balance = self.get_balance(addr).unwrap_or(0);

        let new_balance = balance * mul_by as i128;

        self.balance.insert(addr.clone(), new_balance);
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut Self as _
    }
}

unsafe fn extract_host<'a>(raw_ctx: *mut c_void) -> &'a mut Host {
    let host = api::svm_instance_context_host_get(raw_ctx);
    svm_common::from_raw_mut::<Host>(host)
}

unsafe fn extract_reg<'a>(raw_ctx: *mut c_void, reg_bits: u32, reg_idx: u32) -> &'a mut Register {
    use wasmer_runtime_core::vm::Ctx as WasmerCtx;

    let ctx = svm_common::from_raw_mut::<WasmerCtx>(raw_ctx);

    svm_runtime::helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx)
}

unsafe extern "C" fn inc_balance(ctx: *mut c_void, addition: i64, reg_bits: u32, reg_idx: u32) {
    let host = extract_host(ctx);
    let reg = extract_reg(ctx, reg_bits, reg_idx);

    let addr = Address::from(reg.as_ptr());
    host.inc_balance(&addr, addition);
}

unsafe extern "C" fn mul_balance(ctx: *mut c_void, mul_by: i64, reg_bits: u32, reg_idx: u32) {
    let host = extract_host(ctx);
    let reg = extract_reg(ctx, reg_bits, reg_idx);

    let addr = Address::from(reg.as_ptr());
    host.mul_balance(&addr, mul_by);
}

unsafe fn create_imports() -> (Vec<*const svm_import_t>, u32) {
    let inc_balance_import = testing::import_func_create(
        "env",
        "inc_balance",
        inc_balance as _,
        vec![
            svm_value_type::SVM_I64,
            svm_value_type::SVM_I32,
            svm_value_type::SVM_I32,
        ],
        vec![],
    );

    let mul_balance_import = testing::import_func_create(
        "env",
        "mul_balance",
        mul_balance as _,
        vec![
            svm_value_type::SVM_I64,
            svm_value_type::SVM_I32,
            svm_value_type::SVM_I32,
        ],
        vec![],
    );

    let imports = vec![inc_balance_import, mul_balance_import];
    let imports_len = imports.len() as u32;

    (imports, imports_len)
}

fn deploy_template_bytes(version: u32, name: &str, page_count: u16, wasm: &str) -> (Vec<u8>, u32) {
    let bytes = svm_runtime::testing::build_template(version, name, page_count, wasm);
    let bytes_len = bytes.len() as u32;

    (bytes, bytes_len)
}

fn spawn_app_bytes(
    version: u32,
    template_addr: *const c_void,
    ctor_buf: &Vec<Vec<u8>>,
    ctor_args: &Vec<WasmValue>,
) -> (Vec<u8>, u32) {
    let template_addr: &Address = unsafe { svm_common::from_raw::<Address>(template_addr) };

    let bytes = svm_runtime::testing::build_app(version, template_addr, ctor_buf, ctor_args);
    let bytes_len = bytes.len() as u32;

    (bytes, bytes_len)
}

fn exec_app_bytes(
    version: u32,
    app_addr: *const c_void,
    func_name: &str,
    func_buf: &Vec<Vec<u8>>,
    func_args: &Vec<WasmValue>,
) -> (Vec<u8>, u32) {
    let app_addr: &Address = unsafe { svm_common::from_raw::<Address>(app_addr) };

    let bytes =
        svm_runtime::testing::build_app_tx(version, app_addr, func_name, func_buf, func_args);
    let bytes_len = bytes.len() as u32;

    (bytes, bytes_len)
}

fn host_ctx_bytes(version: u32, fields: HashMap<u32, Vec<u8>>) -> (Vec<u8>, u32) {
    let bytes = svm_runtime::testing::build_host_ctx(version, fields);
    let bytes_len = bytes.len() as u32;

    (bytes, bytes_len)
}

fn exec_app_args() -> (Address, Address, u64, Vec<Vec<u8>>, Vec<WasmValue>, State) {
    let sender = Address::of("sender");

    let user = Address::of("user");
    let user_bytes = user.bytes().to_vec();
    let func_buf = vec![user_bytes];

    let addition = 2;
    let func_args = vec![WasmValue::I64(addition)];

    let state = State::empty();

    (sender, user, addition, func_buf, func_args, state)
}

#[test]
fn runtime_ffi_exec_app() {
    unsafe {
        do_ffi_exec_app();
    }
}

unsafe fn do_ffi_exec_app() {
    let version: u32 = 0;

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
    let author = Address::of("author");
    let code = include_str!("wasm/update-balance.wast");
    let page_count = 10;
    let (hctx_bytes, hctx_len) = host_ctx_bytes(version, hashmap! {});
    let (bytes, bytes_len) = deploy_template_bytes(version, "MyTemplate #1", page_count, code);
    let mut template = std::ptr::null_mut();

    let res = api::svm_deploy_template(
        &mut template,
        runtime,
        author.as_ptr() as _,
        hctx_bytes.as_ptr() as _,
        hctx_len as _,
        bytes.as_ptr() as _,
        bytes_len,
    );
    assert_eq!(true, res.as_bool());

    // 3) spawn app
    let mut app_addr = std::ptr::null_mut();
    let creator = Address::of("creator");
    let ctor_buf = vec![];
    let ctor_args = vec![];
    let (hctx_bytes, hctx_len) = host_ctx_bytes(version, hashmap! {});
    let (bytes, bytes_len) = spawn_app_bytes(version, template as _, &ctor_buf, &ctor_args);
    let res = api::svm_spawn_app(
        &mut app_addr,
        runtime,
        creator.as_ptr() as _,
        hctx_bytes.as_ptr() as _,
        hctx_len as _,
        bytes.as_ptr() as _,
        bytes_len,
    );
    assert_eq!(true, res.as_bool());

    // 4) execute app
    let (sender, user, addition, func_buf, func_args, state) = exec_app_args();
    let (bytes, bytes_len) = exec_app_bytes(version, app_addr, "run", &func_buf, &func_args);

    // 4.1) parse bytes into in-memory `AppTransaction`
    let mut app_tx = std::ptr::null_mut();
    let res = api::svm_parse_exec_app(
        &mut app_tx,
        runtime,
        sender.as_ptr() as _,
        bytes.as_ptr() as _,
        bytes_len,
    );
    assert_eq!(true, res.as_bool());

    // 4.2) execute the app-transaction
    let init_balance = 100;
    host.set_balance(&user, init_balance);

    let nonce = 3;
    const NONCE_INDEX: u32 = 0;

    // we set field index `2` with a value called `nonce` (one byte).
    let (host_ctx_bytes, host_ctx_len) =
        host_ctx_bytes(version, hashmap! { NONCE_INDEX => vec![nonce] });

    let mut receipt = std::ptr::null_mut();
    let mut receipt_length = 0;
    let res = api::svm_exec_app(
        &mut receipt,
        &mut receipt_length,
        runtime,
        app_tx,
        svm_common::into_raw(state),
        host_ctx_bytes.as_ptr() as _,
        host_ctx_len as _,
    );
    assert_eq!(true, res.as_bool());

    let expected = (init_balance + addition as i128) * (nonce as i128);
    let actual = host.get_balance(&user).unwrap();

    assert_eq!(expected, actual);
}
