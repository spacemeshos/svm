#![allow(unused)]

extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;

use std::convert::TryFrom;
use std::ffi::c_void;

use svm_codec::api::raw;
use svm_ffi::{svm_byte_array, svm_env_t, svm_trap_t};
use svm_layout::DataLayout;
use svm_runtime::{testing::WasmFile, vmcalls, Context};
use svm_types::{Address, State, WasmType, WasmValue};

use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;

/// We should land here when `trampoline` has been called with `host_env` containing
/// a function index equaling to `COUNTER_MUL_FN_INDEX`
fn counter_mul(ctx: &mut Context, args: &[WasmValue]) -> Result<Vec<WasmValue>, &'static str> {
    // assert_eq!(args.len(), 2);

    // let var_id = args[0].unwrap_i32() as u32;
    // let mul = args[1].unwrap_i32() as u32;

    // let old = vmcalls::get32(ctx, var_id);
    // let new = old * mul;

    // let results = vec![Val::I32(new as i32)];
    // Ok(results)

    Ok(Vec::new())
}

/// This struct will serve as our `host_env`.
/// We use here integers as function identifiers to be called by the `trampoline`.
#[derive(Debug)]
#[repr(C)]
struct func_index_t(u32);

type Callback = fn(&mut Context, &[WasmValue]) -> Result<Vec<WasmValue>, &'static str>;

const COUNTER_MUL_FN_INDEX: u32 = 123;

fn wasm_trap(err: String) -> *mut svm_trap_t {
    let trap: svm_trap_t = err.into();

    // this heap-allocated memory will be released by SVM.
    // (See: `ExternImport#wasmer_export`)

    Box::into_raw(Box::new(trap))
}

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

/// The memory for results has already been zero-allocated, we're left with filling-in
/// the values given by `values` parameter.
///
// unsafe fn copy_results(results: *mut wasm_val_vec_t, values: &[Val]) {
// let values: Vec<wasm_val_t> = values
//     .iter()
//     .map(|v| wasm_val_t::try_from(v).unwrap())
//     .collect();

// let results: &mut wasm_val_vec_t = &mut *results;

// results.size = values.len();

// for (i, val) in values.iter().enumerate() {
//     let out_ptr = results.data.add(i);

//     wasm_val_copy(out_ptr, val);
// }
// }

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
/// In case the `trampoline` failed, a pointer to heap-allocated trap will be propagated back to SVM.
/// SVM will be responsible of deallocating that memory pointed by that `svm_trap_t`.
#[no_mangle]
unsafe extern "C" fn trampoline(
    env: *mut c_void,
    args: *const svm_byte_array,
    results: *mut svm_byte_array,
) -> *mut svm_trap_t {
    let env: &svm_env_t = env.into();
    let func_idx = env.host_env::<func_index_t>();
    let callback = func_index_to_callback(func_idx);

    if let Some(callback) = callback {
        let args = prepare_args(args).unwrap();
        let ctx = env.inner_mut();

        match callback(ctx, &args) {
            Ok(values) => {
                /// We copy the values returned by `callback` to `results`.
                /// This copying operation must not fail (otherwise it's an undefined-behavior).
                // copy_results(results, &values);

                /// since `callback` didn't error, we return a `NULL` pointer signaling
                // that there was no trap has occurred.
                return std::ptr::null_mut();
            }
            Err(err) => wasm_trap(err.to_string()),
        }
    } else {
        /// `trampoline` has nowhere to jump.
        /// (There is no function associated with `func_idx.0` integer).
        let err = format!("Unknown host function indexed: {}", func_idx.0);

        wasm_trap(err)
    }
}

unsafe fn create_imports() -> *const c_void {
    let mut imports = std::ptr::null_mut();
    let length = 1;

    let res = api::svm_imports_alloc(&mut imports, length);
    assert!(res.is_ok());

    // `counter_mul` import
    let func_ptr: *const c_void = trampoline as _;

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
        namespace.into(),
        import_name.into(),
        func_ptr,
        host_env,
        params.into(),
        returns.into(),
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
    let mut state_kv = std::ptr::null_mut();
    let mut runtime = std::ptr::null_mut();
    let imports = create_imports();
    let mut error = svm_byte_array::default();

    let res = api::svm_memory_state_kv_create(&mut state_kv);
    assert!(res.is_ok());

    let res = api::svm_memory_runtime_create(&mut runtime, state_kv, imports, &mut error);
    assert!(res.is_ok());

    // 2) deploy app-template
    let author = Address::of("author").into();
    let wasm = include_bytes!("wasm/counter.wasm");

    // raw template
    let bytes = deploy_template_bytes(version, "My Template", wasm);
    let template_bytes: svm_byte_array = bytes.into();

    let mut template_receipt = svm_byte_array::default();
    let res = api::svm_deploy_template(
        &mut template_receipt,
        runtime,
        template_bytes,
        author,
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // extract the `template-address` out of theh receipt
    let receipt = raw::decode_receipt(template_receipt.clone().into()).into_deploy_template();
    let template_addr: &Address = receipt.get_template_addr().inner();
    let template_addr: svm_byte_array = template_addr.into();

    // 3) spawn app
    let name = "My App";
    let spawner = Address::of("spawner").into();
    let ctor_name = "initialize";
    let counter_init: u32 = 10;

    let mut calldata = Vec::new();
    counter_init.encode(&mut calldata);

    // raw `spawn-app`
    let bytes = spawn_app_bytes(version, &template_addr, name, ctor_name, &calldata);
    let app_bytes: svm_byte_array = bytes.into();

    let mut spawn_receipt = svm_byte_array::default();

    let res = api::svm_spawn_app(
        &mut spawn_receipt,
        runtime,
        app_bytes,
        spawner,
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    // extracts the spawned-app `Address` and initial `State`.
    let receipt = raw::decode_receipt(spawn_receipt.clone().into()).into_spawn_app();
    assert_eq!(receipt.success, true);
    let app_addr = receipt.get_app_addr().inner();
    let app_addr: svm_byte_array = app_addr.into();

    let init_state = receipt.get_init_state();
    let init_state: svm_byte_array = init_state.into();

    // 4) execute app
    let func_name = "add_and_mul";
    let add = 5u32;
    let mul = 3u32;

    let mut calldata = Vec::new();

    add.encode(&mut calldata);
    mul.encode(&mut calldata);

    let bytes = exec_app_bytes(version, &app_addr, func_name, &calldata);
    let tx_bytes: svm_byte_array = bytes.into();

    // 4.1) validates tx and extracts its `App`'s `Address`
    let mut app_addr = svm_byte_array::default();
    let res = api::svm_validate_tx(&mut app_addr, runtime, tx_bytes.clone(), &mut error);
    assert!(res.is_ok());

    // 4.2) execute the app-transaction
    let mut exec_receipt = svm_byte_array::default();

    let res = api::svm_exec_app(
        &mut exec_receipt,
        runtime,
        tx_bytes,
        init_state.clone(),
        gas_metering,
        gas_limit,
        &mut error,
    );
    assert!(res.is_ok());

    let receipt = raw::decode_receipt(exec_receipt.clone().into()).into_exec_app();
    assert_eq!(receipt.success, true);

    let bytes = receipt.get_returndata();
    let mut returndata = ReturnData::new(bytes.as_slice());

    let [a, b, c]: [u32; 3] = returndata.next_1();

    assert_eq!(
        (a, b, c),
        (counter_init, counter_init + add, (counter_init + add) * mul)
    );

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
