extern crate svm_runtime_c_api;

use maplit::hashmap;

use svm_runtime_c_api as api;
use svm_runtime_c_api::svm_byte_array;

use std::{collections::HashMap, ffi::c_void};

use svm_codec::api::raw;
use svm_layout::DataLayout;
use svm_types::{Address, State, WasmValue};

unsafe fn create_imports() -> *const c_void {
    let mut imports = std::ptr::null_mut();
    let length = 0;

    let res = api::svm_imports_alloc(&mut imports, length);
    assert!(res.is_ok());

    imports as _
}

fn deploy_template_bytes(version: u32, name: &str, wasm: &str) -> (Vec<u8>, u32) {
    let is_wast = true;
    let data: DataLayout = vec![4].into();
    let bytes = svm_runtime::testing::build_template(version, name, data, wasm, is_wast);
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
    let app_addr: &[u8] = app_addr.into();
    let app_addr = Address::from(app_addr).into();

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
    let host = std::ptr::null_mut();
    let mut state_kv = std::ptr::null_mut();
    let mut runtime = std::ptr::null_mut();
    let imports = create_imports();
    let mut error = svm_byte_array::default();

    let res = api::svm_memory_state_kv_create(&mut state_kv);
    assert!(res.is_ok());

    let res = api::svm_memory_runtime_create(&mut runtime, state_kv, host, imports, &mut error);
    assert!(res.is_ok());

    // 2) deploy app-template
    let author = Address::of("author").into();
    let code = include_str!("wasm/counter.wast");

    // raw `host ctx`
    let (bytes, length) = host_ctx_bytes(version, hashmap! {});
    let host_ctx = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    // raw template
    let (bytes, length) = deploy_template_bytes(version, "My Template", code);
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
    let receipt = raw::decode_receipt(template_receipt.into()).into_deploy_template();
    let template_addr: &Address = receipt.get_template_addr().inner();
    let template_addr: svm_byte_array = template_addr.into();

    // 3) spawn app
    let spawner = Address::of("spawner").into();
    let ctor_idx = 0;
    let ctor_buf = vec![];
    let ctor_args = vec![];

    // raw `spawn-app`
    let (bytes, length) = spawn_app_bytes(version, &template_addr, ctor_idx, &ctor_buf, &ctor_args);
    let app_bytes = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    let mut spawn_receipt = svm_byte_array::default();

    let res = api::svm_spawn_app(
        &mut spawn_receipt,
        runtime,
        app_bytes,
        spawner,
        host_ctx,
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // extracts the spawned-app `Address` and initial `State`.
    let receipt = raw::decode_receipt(spawn_receipt.into()).into_spawn_app();
    let app_addr: &Address = receipt.get_app_addr().inner();
    let app_addr: svm_byte_array = app_addr.into();

    let init_state: &State = receipt.get_init_state();
    let init_state: svm_byte_array = init_state.into();

    // 4) execute app
    let func_idx = 1;
    let func_buf = vec![];
    let func_args = vec![WasmValue::I64(10)];

    let (bytes, length) = exec_app_bytes(version, &app_addr, func_idx, &func_buf, &func_args);
    let tx_bytes = svm_byte_array {
        bytes: bytes.as_ptr(),
        length: length,
    };

    // 4.1) validates tx and extracts its `App`'s `Address`
    let mut app_addr = svm_byte_array::default();
    let res = api::svm_validate_tx(&mut app_addr, runtime, tx_bytes, &mut error);
    assert!(res.is_ok());

    // 4.2) execute the app-transaction
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

    let receipt = raw::decode_receipt(exec_receipt.into()).into_exec_app();
    assert_eq!(receipt.get_returns(), &vec![WasmValue::I64(10)]);

    let _ = api::svm_byte_array_destroy(template_addr);
    let _ = api::svm_byte_array_destroy(app_addr);
    let _ = api::svm_byte_array_destroy(init_state);
    let _ = api::svm_byte_array_destroy(template_receipt);
    let _ = api::svm_byte_array_destroy(spawn_receipt);
    let _ = api::svm_byte_array_destroy(exec_receipt);
    let _ = api::svm_imports_destroy(imports);
    let _ = api::svm_runtime_destroy(runtime);
    let _ = api::svm_state_kv_destroy(state_kv);
}
