extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;
use svm_runtime_c_api::{svm_byte_array, testing};

use maplit::hashmap;

use std::{collections::HashMap, ffi::c_void};

use svm_app::types::{WasmType, WasmValue};
use svm_common::Address;
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

unsafe fn extract_reg<'a>(raw_ctx: *mut c_void, reg_bits: i32, reg_idx: i32) -> &'a mut Register {
    use wasmer_runtime_core::vm::Ctx as WasmerCtx;

    let ctx = svm_common::from_raw_mut::<WasmerCtx>(raw_ctx);

    svm_runtime::helpers::wasmer_data_reg(ctx.data, reg_bits as u32, reg_idx as u32)
}

unsafe extern "C" fn inc_balance(ctx: *mut c_void, addition: i64, reg_bits: i32, reg_idx: i32) {
    let host = extract_host(ctx);
    let reg = extract_reg(ctx, reg_bits, reg_idx);

    let addr = Address::from(reg.as_ptr());
    host.inc_balance(&addr, addition);
}

unsafe extern "C" fn mul_balance(ctx: *mut c_void, mul_by: i64, reg_bits: i32, reg_idx: i32) {
    let host = extract_host(ctx);
    let reg = extract_reg(ctx, reg_bits, reg_idx);

    let addr = Address::from(reg.as_ptr());
    host.mul_balance(&addr, mul_by);
}

unsafe fn create_imports() -> *const c_void {
    let mut imports = std::ptr::null_mut();
    let length = 2;

    let res = api::svm_imports_alloc(&mut imports, length);
    assert!(res.is_ok());

    testing::import_func_create(
        imports,
        "env",
        "inc_balance",
        inc_balance as _,
        vec![WasmType::I64, WasmType::I32, WasmType::I32].into(),
        Vec::<WasmType>::new().into(),
    );

    testing::import_func_create(
        imports,
        "env",
        "mul_balance",
        mul_balance as _,
        vec![WasmType::I64, WasmType::I32, WasmType::I32].into(),
        Vec::<WasmType>::new().into(),
    );

    imports as _
}

fn deploy_template_bytes(version: u32, name: &str, page_count: u16, wasm: &str) -> (Vec<u8>, u32) {
    let is_wast = true;
    let bytes = svm_runtime::testing::build_template(version, name, page_count, wasm, is_wast);
    let length = bytes.len() as u32;

    (bytes, length)
}

fn spawn_app_bytes(
    version: u32,
    template_addr: &svm_byte_array,
    ctor_idx: u16,
    ctor_buf: &Vec<u8>,
    ctor_args: &Vec<WasmValue>,
) -> (Vec<u8>, u32) {
    let template_addr = Address::from(*&template_addr.bytes as *const c_void).into();

    let bytes =
        svm_runtime::testing::build_app(version, &template_addr, ctor_idx, ctor_buf, ctor_args);
    let length = bytes.len() as u32;

    (bytes, length)
}

fn exec_app_bytes(
    version: u32,
    app_addr: &svm_byte_array,
    func_idx: u16,
    func_buf: &Vec<u8>,
    func_args: &Vec<WasmValue>,
) -> (Vec<u8>, u32) {
    let app_addr = Address::from(*&app_addr.bytes as *const c_void).into();

    let bytes =
        svm_runtime::testing::build_app_tx(version, &app_addr, func_idx, func_buf, func_args);

    let length = bytes.len() as u32;

    (bytes, length)
}

fn host_ctx_bytes(version: u32, fields: HashMap<u32, Vec<u8>>) -> (Vec<u8>, u32) {
    let bytes = svm_runtime::testing::build_host_ctx(version, fields);
    let length = bytes.len() as u32;

    (bytes, length)
}

fn exec_app_args() -> (Address, u64, u16, Vec<u8>, Vec<WasmValue>) {
    let func_idx = 1;

    let user = Address::of("user");
    let func_buf = user.bytes().to_vec();

    let addition = 2;
    let func_args = vec![WasmValue::I64(addition)];

    (user, addition, func_idx, func_buf, func_args)
}

#[test]
fn svm_runtime_exec_app() {
    unsafe {
        test_svm_runtime();
    }
}

unsafe fn test_svm_runtime() {
    let version: u32 = 0;
    let gas_metering = false;
    let gas_limit = 0;

    // 1) init runtime
    let mut host = Host::new();
    let mut kv = std::ptr::null_mut();
    let mut runtime = std::ptr::null_mut();
    let imports = create_imports();
    let mut error = svm_byte_array::default();

    let res = api::svm_memory_kv_create(&mut kv);
    assert!(res.is_ok());

    let res =
        api::svm_memory_runtime_create(&mut runtime, kv, host.as_mut_ptr(), imports, &mut error);

    assert!(res.is_ok());

    // 2) deploy app-template
    let author = Address::of("author").into();
    let code = include_str!("wasm/update-balance.wast");
    let page_count = 10;

    // raw `host ctx`
    let (bytes, length) = host_ctx_bytes(version, hashmap! {});
    let host_ctx = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    // raw template
    let (bytes, length) = deploy_template_bytes(version, "My Template", page_count, code);
    let template_bytes = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    let mut template_receipt = svm_byte_array::default();
    let res = api::svm_deploy_template(
        &mut template_receipt,
        runtime,
        template_bytes,
        author,
        host_ctx,
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // extract the `template-address` out of theh receipt
    let mut template_addr = svm_byte_array::default();
    let res = api::svm_template_receipt_addr(&mut template_addr, template_receipt, &mut error);
    assert!(res.is_ok());

    // 3) spawn app
    let creator = Address::of("creator").into();
    let ctor_idx = 0;
    let ctor_buf = vec![];
    let ctor_args = vec![];

    // raw `host ctx`
    let (bytes, length) = host_ctx_bytes(version, hashmap! {});
    let host_ctx = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    // raw `spawn-app`
    let (bytes, length) = spawn_app_bytes(version, &template_addr, ctor_idx, &ctor_buf, &ctor_args);
    let app_bytes = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    let mut app_receipt = svm_byte_array::default();

    let res = api::svm_spawn_app(
        &mut app_receipt,
        runtime,
        app_bytes,
        creator,
        host_ctx,
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // extracts the spawned-app `Address` and initial `State`.
    let mut app_addr = svm_byte_array::default();
    let mut init_state = svm_byte_array::default();

    let res = api::svm_app_receipt_addr(&mut app_addr, app_receipt, &mut error);
    assert!(res.is_ok());

    let res = api::svm_app_receipt_state(&mut init_state, app_receipt, &mut error);
    assert!(res.is_ok());

    // 4) execute app
    let (user, addition, func_idx, func_buf, func_args) = exec_app_args();
    let (bytes, length) = exec_app_bytes(version, &app_addr, func_idx, &func_buf, &func_args);
    let tx_bytes = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    // 4.1) validates tx and extract its `app-address`.
    let mut app_addr = svm_byte_array::default();
    let res = api::svm_validate_tx(&mut app_addr, runtime, tx_bytes, &mut error);
    assert!(res.is_ok());

    // // 4.2) execute the app-transaction
    let init_balance = 100;
    host.set_balance(&user, init_balance);

    let nonce = 3;
    const NONCE_INDEX: u32 = 0;

    // we set field index `2` with a value called `nonce` (one byte).
    let (bytes, length) = host_ctx_bytes(version, hashmap! { NONCE_INDEX => vec![nonce] });
    let host_ctx = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    let mut exec_receipt = svm_byte_array::default();

    let res = api::svm_exec_app(
        &mut exec_receipt,
        runtime,
        tx_bytes,
        init_state,
        host_ctx,
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    let expected = (init_balance + addition as i128) * (nonce as i128);
    let actual = host.get_balance(&user).unwrap();

    assert_eq!(expected, actual);

    let _ = api::svm_byte_array_destroy(template_addr);
    let _ = api::svm_byte_array_destroy(app_addr);
    let _ = api::svm_byte_array_destroy(init_state);
    let _ = api::svm_byte_array_destroy(template_receipt);
    let _ = api::svm_byte_array_destroy(app_receipt);
    let _ = api::svm_byte_array_destroy(exec_receipt);
    let _ = api::svm_imports_destroy(imports);
    let _ = api::svm_runtime_destroy(runtime);
}
