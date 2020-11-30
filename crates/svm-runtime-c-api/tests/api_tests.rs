#![allow(unused)]

extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;

use std::convert::TryFrom;
use std::ffi::c_void;

use svm_codec::api::raw;
use svm_ffi::TypeIdOrStr;
use svm_ffi::{svm_byte_array, svm_env_t};
use svm_layout::DataLayout;
use svm_runtime::{testing::WasmFile, vmcalls, Context};
use svm_types::{Address, State, WasmType, WasmValue};

use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;

static TEST_STRING_TY: TypeIdOrStr = TypeIdOrStr::Str("test String");
static AUTHOR: TypeIdOrStr = TypeIdOrStr::Str("author");
static SPAWNER: TypeIdOrStr = TypeIdOrStr::Str("spawner");
static SENDER: TypeIdOrStr = TypeIdOrStr::Str("sender");
static TEMPLATE_ADDR: TypeIdOrStr = TypeIdOrStr::Str("template address");
static APP_ADDR: TypeIdOrStr = TypeIdOrStr::Str("app address");
static INIT_STATE: TypeIdOrStr = TypeIdOrStr::Str("init state");
static DEPLOY_TEMPLATE_TX: TypeIdOrStr = TypeIdOrStr::Str("deploy template tx");
static SPAWN_APP_TX: TypeIdOrStr = TypeIdOrStr::Str("spawn app tx");
static EXEC_APP_TX: TypeIdOrStr = TypeIdOrStr::Str("exec app tx");
static IMPORT_NS: TypeIdOrStr = TypeIdOrStr::Str("import nasmespace");
static IMPORT_NAME: TypeIdOrStr = TypeIdOrStr::Str("import name");
static PARAMS_TYPES: TypeIdOrStr = TypeIdOrStr::Str("import params types");
static RETURNS_TYPES: TypeIdOrStr = TypeIdOrStr::Str("import returns types");

/// We should land here when `trampoline` has been called with `host_env` containing
/// a function index equaling to `COUNTER_MUL_FN_INDEX`
fn counter_mul(ctx: &mut Context, args: &[WasmValue]) -> Result<Vec<WasmValue>, &'static str> {
    assert_eq!(args.len(), 2);

    let var_id = args[0].as_i32().unwrap();
    let mul = args[1].as_i32().unwrap();

    let old = vmcalls::get32(ctx, var_id);
    let new = old * mul;

    let results = vec![WasmValue::I32(new)];

    Ok(results)
}

/// This struct will serve as our `host_env`.
/// We use here integers as function identifiers to be called by the `trampoline`.
#[derive(Debug)]
#[repr(C)]
struct func_index_t(u32);

type Callback = fn(&mut Context, &[WasmValue]) -> Result<Vec<WasmValue>, &'static str>;

const COUNTER_MUL_FN_INDEX: u32 = 123;

/// Given a function identifier stored as part of the `host_env`
/// we can know what Rust native function to call
fn func_index_to_callback(func_idx: &func_index_t) -> Option<Callback> {
    match func_idx.0 {
        COUNTER_MUL_FN_INDEX => Some(counter_mul),
        _ => None,
    }
}

unsafe fn prepare_args(args: *const svm_byte_array) -> Result<Vec<WasmValue>, &'static str> {
    let args: &svm_byte_array = &*args;

    Vec::<WasmValue>::try_from(args).map_err(|_| "Invalid args")
}

unsafe fn wasm_error(msg: String) -> *mut svm_byte_array {
    let template_bytes: svm_byte_array = (TEST_STRING_TY, msg).into();

    api::svm_wasm_error_create(template_bytes)
}

/// The `trampoline` is the actual host function that will be called by SVM running.
/// Each host function will ask SVM to call that `trampoline` function.
///
/// The critical difference between different imports is the `host_env` attached to each import.
/// By placing a function identifier under each `host env` the `trampoline` can figure out to which
/// Rust function to call. In this example we've introduced `func_index_t` (merely an wrapper for an integer)
/// as our `host_env`. Other clients can use the same technique very similarly.
///
/// Each import function will also give its function signature. The `trampoline` will receive under its values specified
/// in the import function signature. In addition to required memory for the `results` will be allocated prior calling `trampoline`
/// such that the `trampoline` will only be left with placing the `results` values.
///
/// In case the `trampoline` failed, a pointer to heap-allocated error message will be propagated back to SVM.
/// SVM will be responsible of deallocating the error message.
unsafe extern "C" fn trampoline(
    env: *mut svm_env_t,
    args: *const svm_byte_array,
    results: *mut svm_byte_array,
) -> *mut svm_byte_array {
    let env: &svm_env_t = &*env;
    let func_idx = env.host_env::<func_index_t>();
    let callback = func_index_to_callback(func_idx);

    if let Some(callback) = callback {
        let args = prepare_args(args).unwrap();
        let ctx = env.inner_mut();

        match callback(ctx, &args) {
            Ok(values) => {
                /// We copy the values returned by `callback` to `results`.
                /// This copying operation must not fail (otherwise it's an undefined-behavior).
                let results: &mut svm_byte_array = &mut *results;
                results.copy_wasm_values(&values);

                /// since `callback` didn't error, we return a `NULL` pointer signaling
                // that there no trap has occurred.
                return std::ptr::null_mut();
            }
            Err(err) => wasm_error(err.to_string()),
        }
    } else {
        /// `trampoline` has nowhere to jump.
        /// (There is no function associated with `func_idx.0` integer).
        let err = format!("Unknown host function indexed: {}", func_idx.0);

        wasm_error(err)
    }
}

unsafe fn create_imports() -> *mut c_void {
    let mut imports = std::ptr::null_mut();
    let length = 1;

    let res = api::svm_imports_alloc(&mut imports, length);
    assert!(res.is_ok());

    // `counter_mul` import
    let func_idx = func_index_t(COUNTER_MUL_FN_INDEX);
    let func_idx: *mut func_index_t = Box::into_raw(Box::new(func_idx));
    let host_env = func_idx as *mut c_void as *const c_void;

    let params = vec![WasmType::I32, WasmType::I32];
    let returns = vec![WasmType::I32];
    let namespace = b"host".to_vec();
    let import_name = b"counter_mul".to_vec();

    let mut error = svm_byte_array::default();

    let res = api::svm_import_func_new(
        imports,
        (IMPORT_NS, namespace).into(),
        (IMPORT_NAME, import_name).into(),
        trampoline,
        host_env,
        (PARAMS_TYPES, params).into(),
        (RETURNS_TYPES, returns).into(),
        &mut error,
    );
    assert!(res.is_ok());

    imports as _
}

fn deploy_template_bytes(version: u32, name: &str, wasm: &[u8]) -> Vec<u8> {
    let data: DataLayout = vec![4].into();

    svm_runtime::testing::build_template(version, name, data, WasmFile::Binary(wasm))
}

fn spawn_app_bytes(
    version: u32,
    template_addr: &svm_byte_array,
    name: &str,
    ctor_name: &str,
    calldata: &Vec<u8>,
) -> Vec<u8> {
    let template_addr = Address::from(*&template_addr.bytes as *const c_void).into();
    svm_runtime::testing::build_app(version, &template_addr, name, ctor_name, calldata)
}

fn exec_app_bytes(
    version: u32,
    app_addr: &svm_byte_array,
    func_name: &str,
    calldata: &Vec<u8>,
) -> Vec<u8> {
    let app_addr: &[u8] = app_addr.into();
    let app_addr = Address::from(app_addr).into();

    svm_runtime::testing::build_app_tx(version, &app_addr, func_name, calldata)
}

fn dbg_snapshot(index: usize) {
    dbg!("===================================================");
    dbg!(format!("Snapshot #{}:", index));

    dbg!(svm_ffi::tracking::snapshot());
    dbg!("===================================================");
}

#[test]
fn svm_runtime_exec_app() {
    unsafe {
        test_svm_runtime();
    }
}

#[cfg(test)]
unsafe fn test_svm_runtime() {
    let guard = svm_ffi::tracking::start();

    let version: u32 = 0;
    let gas_metering = false;
    let gas_limit = 0;

    dbg_snapshot(0);

    // 1) init runtime
    let mut state_kv = std::ptr::null_mut();
    let mut runtime = std::ptr::null_mut();
    let imports = create_imports();
    let mut error = svm_byte_array::default();

    let res = api::svm_memory_state_kv_create(&mut state_kv);
    assert!(res.is_ok());

    let res = api::svm_memory_runtime_create(&mut runtime, state_kv, imports, &mut error);
    assert!(res.is_ok());

    // dbg_snapshot(1);

    // 2) deploy app-template
    let author: svm_byte_array = (AUTHOR, Address::of("author")).into();
    let wasm = include_bytes!("wasm/counter.wasm");

    // raw template
    let template_bytes = deploy_template_bytes(version, "My Template", wasm);
    let template_bytes: svm_byte_array = (DEPLOY_TEMPLATE_TX, template_bytes).into();

    let mut template_receipt = svm_byte_array::default();
    let res = api::svm_deploy_template(
        &mut template_receipt,
        runtime,
        template_bytes.clone(),
        author.clone(),
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // dbg_snapshot(2);

    // extract the `template-address` out of theh receipt
    let receipt = raw::decode_receipt(template_receipt.clone().into()).into_deploy_template();
    let template_addr: &Address = receipt.get_template_addr().inner();
    let template_addr: svm_byte_array = (TEMPLATE_ADDR, template_addr).into();

    // 3) spawn app
    let name = "My App";
    let spawner: svm_byte_array = (SPAWNER, Address::of("spawner")).into();
    let ctor_name = "initialize";
    let counter_init: u32 = 10;

    let mut calldata = Vec::new();
    counter_init.encode(&mut calldata);

    // raw `spawn-app`
    let app_bytes = spawn_app_bytes(version, &template_addr, name, ctor_name, &calldata);
    let app_bytes: svm_byte_array = (SPAWN_APP_TX, app_bytes).into();

    let mut spawn_receipt = svm_byte_array::default();

    let res = api::svm_spawn_app(
        &mut spawn_receipt,
        runtime,
        app_bytes.clone(),
        spawner.clone(),
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // extracts the spawned-app `Address` and initial `State`.
    let receipt = raw::decode_receipt(spawn_receipt.clone().into()).into_spawn_app();
    assert_eq!(receipt.success, true);
    let app_addr = receipt.get_app_addr().inner();
    let app_addr: svm_byte_array = (APP_ADDR, app_addr).into();

    let init_state = receipt.get_init_state();
    let init_state: svm_byte_array = (INIT_STATE, init_state).into();

    // dbg_snapshot(3);

    // 4) execute app
    let func_name = "add_and_mul";
    let add = 5u32;
    let mul = 3u32;

    let mut calldata = Vec::new();

    add.encode(&mut calldata);
    mul.encode(&mut calldata);

    let exec_bytes = exec_app_bytes(version, &app_addr, func_name, &calldata);
    let exec_bytes: svm_byte_array = (EXEC_APP_TX, exec_bytes).into();

    // 4.1) validates tx and extracts its `App`'s `Address`
    let mut derived_app_addr = svm_byte_array::default();
    let res = api::svm_validate_tx(
        &mut derived_app_addr,
        runtime,
        exec_bytes.clone(),
        &mut error,
    );
    assert!(res.is_ok());

    // 4.2) execute the app-transaction
    let mut exec_receipt = svm_byte_array::default();

    let res = api::svm_exec_app(
        &mut exec_receipt,
        runtime,
        exec_bytes.clone(),
        init_state.clone(),
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // dbg_snapshot(4);

    let receipt = raw::decode_receipt(exec_receipt.clone().into()).into_exec_app();
    assert_eq!(receipt.success, true);

    let bytes = receipt.get_returndata();
    let mut returndata = ReturnData::new(bytes.as_slice());

    let [a, b, c]: [u32; 3] = returndata.next_1();

    assert_eq!(
        (a, b, c),
        (counter_init, counter_init + add, (counter_init + add) * mul)
    );

    let _ = api::svm_byte_array_destroy(template_bytes);
    let _ = api::svm_byte_array_destroy(app_bytes);
    let _ = api::svm_byte_array_destroy(exec_bytes);
    let _ = api::svm_byte_array_destroy(author);
    let _ = api::svm_byte_array_destroy(spawner);
    let _ = api::svm_byte_array_destroy(template_addr);
    let _ = api::svm_byte_array_destroy(app_addr);
    let _ = api::svm_byte_array_destroy(init_state);
    let _ = api::svm_byte_array_destroy(template_receipt);
    let _ = api::svm_byte_array_destroy(spawn_receipt);
    let _ = api::svm_byte_array_destroy(exec_receipt);
    let _ = api::svm_imports_destroy(imports);
    let _ = api::svm_runtime_destroy(runtime);
    let _ = api::svm_state_kv_destroy(state_kv);

    // assert_eq!(svm_ffi::tracking::total_live_count(), 0);

    dbg_snapshot(5);

    svm_ffi::tracking::end(guard);

    panic!()
}
