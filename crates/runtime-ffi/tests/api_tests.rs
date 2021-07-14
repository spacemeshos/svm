use maplit::hashmap;

use std::convert::TryFrom;
use std::ffi::c_void;
use std::{collections::HashMap, vec};

use svm_codec::receipt;
use svm_ffi::{svm_byte_array, tracking};
use svm_layout::FixedLayout;
use svm_runtime::testing::WasmFile;
use svm_runtime_ffi as api;
use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;
use svm_types::{Address, Type};

static TEST_STRING_TY: Type = Type::Str("test String");
static DEPLOYER: Type = Type::Str("deployer");
static SPAWNER: Type = Type::Str("spawner");
static SENDER: Type = Type::Str("sender");
static TEMPLATE_ADDR: Type = Type::Str("template address");
static APP_ADDR: Type = Type::Str("app address");
static INIT_STATE: Type = Type::Str("init state");
static DEPLOY_TEMPLATE_TX: Type = Type::Str("deploy template tx");
static SPAWN_APP_TX: Type = Type::Str("spawn app tx");
static EXEC_APP_TX: Type = Type::Str("exec app tx");
static PARAMS_TYPES: Type = Type::Str("import params types");
static RETURNS_TYPES: Type = Type::Str("import returns types");

unsafe fn wasm_error(msg: String) -> *mut svm_byte_array {
    let msg: svm_byte_array = (TEST_STRING_TY, msg).into();
    let err = api::svm_wasm_error_create(msg.clone());

    msg.destroy();

    err
}

fn deploy_template_bytes(code_version: u32, name: &str, ctors: &[String], wasm: &[u8]) -> Vec<u8> {
    let data: FixedLayout = vec![4].into();

    svm_runtime::testing::build_template(code_version, name, data, ctors, WasmFile::Binary(wasm))
}

fn spawn_app_bytes(
    template_addr: &svm_byte_array,
    name: &str,
    ctor_name: &str,
    calldata: &[u8],
) -> Vec<u8> {
    let template_addr = Address::from(*&template_addr.bytes as *const c_void).into();
    svm_runtime::testing::build_app(&template_addr, name, ctor_name, calldata)
}

fn exec_app_bytes(app_addr: &svm_byte_array, func_name: &str, calldata: &[u8]) -> Vec<u8> {
    let app_addr: &[u8] = app_addr.into();
    let app_addr = Address::from(app_addr).into();

    svm_runtime::testing::build_transaction(&app_addr, func_name, calldata)
}

#[test]
fn svm_resources_tracking() {
    unsafe {
        tracking::set_tracking_on();

        let ty1 = Type::Str("#1");
        let s1 = "Hello".to_string();
        let hello: svm_byte_array = (ty1, s1).into();

        let ty2 = Type::Str("#2");
        let s2 = "World".to_string();
        let world: svm_byte_array = (ty2, s2).into();

        let s3 = "New World".to_string();
        let new_world: svm_byte_array = (ty2, s3).into();

        let snapshot = tracking::take_snapshot();

        assert_eq!(api::svm_total_live_resources(), 3);

        let iter = api::svm_resource_iter_new();

        let r1 = &mut *api::svm_resource_iter_next(iter);
        let r2 = &mut *api::svm_resource_iter_next(iter);

        let raw_ty1 = &mut *api::svm_resource_type_name_resolve(r1.type_id);
        let raw_ty2 = &mut *api::svm_resource_type_name_resolve(r2.type_id);

        let ty1 = String::try_from(raw_ty1.clone()).unwrap();
        let ty2 = String::try_from(raw_ty2.clone()).unwrap();

        let mut map = HashMap::new();
        map.insert(ty1, r1.count);
        map.insert(ty2, r2.count);

        assert_eq!(
            map,
            hashmap! { "#1".to_string() => 1, "#2".to_string() => 2}
        );

        let r3 = api::svm_resource_iter_next(iter);
        assert_eq!(r3, std::ptr::null_mut());

        api::svm_resource_type_name_destroy(raw_ty1);
        api::svm_resource_type_name_destroy(raw_ty2);

        api::svm_resource_destroy(r1);
        api::svm_resource_destroy(r2);
        api::svm_resource_iter_destroy(iter);

        api::svm_byte_array_destroy(hello);
        api::svm_byte_array_destroy(world);
        api::svm_byte_array_destroy(new_world);

        assert_eq!(api::svm_total_live_resources(), 0);

        tracking::set_tracking_off();
    }
}

#[ignore]
#[test]
fn svm_runtime_failure() {
    unsafe {
        tracking::set_tracking_on();

        assert_eq!(tracking::total_live(), 0);

        let code_version = 1;
        let gas_metering = false;
        let gas_limit = 0;

        // 1) init Runtime
        let mut state_kv = std::ptr::null_mut();
        let mut runtime = std::ptr::null_mut();
        let mut error = svm_byte_array::default();

        let res = api::svm_memory_state_kv_create(&mut state_kv);
        assert!(res.is_ok());

        let res = api::svm_memory_runtime_create(&mut runtime, state_kv, &mut error);
        assert!(res.is_ok());

        // 2) `Deploy Template`
        let deployer: svm_byte_array = (DEPLOYER, Address::of("deployer")).into();
        let wasm = include_bytes!("wasm/failure.wasm");

        // raw `Deploy Template`
        let ctors = vec!["initialize".to_string()];
        let msg = deploy_template_bytes(code_version, "My Template", &ctors, wasm);
        let msg: svm_byte_array = (DEPLOY_TEMPLATE_TX, msg).into();

        let mut template_receipt = svm_byte_array::default();
        let res = api::svm_deploy_template(
            &mut template_receipt,
            runtime,
            msg.clone(),
            deployer.clone(),
            gas_metering,
            gas_limit,
            &mut error,
        );
        assert!(res.is_ok());

        // extract the `Template Address` out of the `Receipt`
        let receipt =
            receipt::decode_receipt(template_receipt.clone().into()).into_deploy();
        let template_addr: &Address = receipt.template_addr().inner();
        let template_addr: svm_byte_array = (TEMPLATE_ADDR, template_addr).into();

        // 3) `Spawn App`
        let name = "My App";
        let spawner: svm_byte_array = (SPAWNER, Address::of("spawner")).into();
        let ctor_name = "initialize";
        let calldata = vec![];

        // raw `Spawn App`
        let version = 0;
        let app_bytes = spawn_app_bytes(&template_addr, name, ctor_name, &calldata);
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
        let receipt = receipt::decode_receipt(spawn_receipt.clone().into()).into_spawn_app();
        assert_eq!(receipt.success, true);

        let app_addr = receipt.app_addr().inner();
        let app_addr: svm_byte_array = (APP_ADDR, app_addr).into();

        let init_state = receipt.init_state();
        let init_state: svm_byte_array = (INIT_STATE, init_state).into();

        // 4) execute app
        let func_name = "fail";
        let mut calldata = vec![];

        let exec_bytes = exec_app_bytes(&app_addr, func_name, &calldata);
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

        let receipt = receipt::decode_receipt(exec_receipt.clone().into()).into_exec_app();
        assert_eq!(receipt.success, false);

        assert_ne!(tracking::total_live(), 0);

        let _ = api::svm_byte_array_destroy(msg);
        let _ = api::svm_byte_array_destroy(app_bytes);
        let _ = api::svm_byte_array_destroy(exec_bytes);
        let _ = api::svm_byte_array_destroy(deployer);
        let _ = api::svm_byte_array_destroy(spawner);
        let _ = api::svm_byte_array_destroy(template_addr);
        let _ = api::svm_byte_array_destroy(app_addr);
        let _ = api::svm_byte_array_destroy(derived_app_addr);
        let _ = api::svm_byte_array_destroy(init_state);
        let _ = api::svm_byte_array_destroy(template_receipt);
        let _ = api::svm_byte_array_destroy(spawn_receipt);
        let _ = api::svm_byte_array_destroy(exec_receipt);
        let _ = api::svm_runtime_destroy(runtime);
        let _ = api::svm_state_kv_destroy(state_kv);

        assert_eq!(tracking::total_live(), 0);

        tracking::set_tracking_off();
    }
}

#[ignore]
#[test]
fn svm_runtime_success() {
    unsafe {
        tracking::set_tracking_on();

        assert_eq!(tracking::total_live(), 0);

        let code_version = 0;
        let gas_metering = false;
        let gas_limit = 0;

        // 1) init runtime
        let mut state_kv = std::ptr::null_mut();
        let mut runtime = std::ptr::null_mut();
        let mut error = svm_byte_array::default();

        let res = api::svm_memory_state_kv_create(&mut state_kv);
        assert!(res.is_ok());

        let res = api::svm_memory_runtime_create(&mut runtime, state_kv, &mut error);
        assert!(res.is_ok());

        // 2) deploy Template
        let deployer: svm_byte_array = (DEPLOYER, Address::of("deployer")).into();
        let ctors = vec!["initialize".to_string()];
        let wasm = include_bytes!("wasm/counter.wasm");

        // raw template
        let msg = deploy_template_bytes(code_version, "My Template", &ctors, wasm);
        let msg: svm_byte_array = (DEPLOY_TEMPLATE_TX, msg).into();

        let mut template_receipt = svm_byte_array::default();
        let res = api::svm_deploy_template(
            &mut template_receipt,
            runtime,
            msg.clone(),
            deployer.clone(),
            gas_metering,
            gas_limit,
            &mut error,
        );
        assert!(res.is_ok());

        // extract the `template-address` out of the receipt
        let receipt =
            receipt::decode_receipt(template_receipt.clone().into()).into_deploy();
        let template_addr: &Address = receipt.template_addr().inner();
        let template_addr: svm_byte_array = (TEMPLATE_ADDR, template_addr).into();

        // 3) spawn app
        let name = "My App";
        let spawner: svm_byte_array = (SPAWNER, Address::of("spawner")).into();
        let ctor_name = "initialize";
        let counter_init: u32 = 10;

        let mut calldata = svm_sdk::Vec::with_capacity(1000);
        counter_init.encode(&mut calldata);

        // raw `spawn-app`
        let app_bytes = spawn_app_bytes(&template_addr, name, ctor_name, calldata.as_slice());
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
        let receipt = receipt::decode_receipt(spawn_receipt.clone().into()).into_spawn_app();
        assert_eq!(receipt.success, true);
        let app_addr = receipt.app_addr().inner();
        let app_addr: svm_byte_array = (APP_ADDR, app_addr).into();

        let init_state = receipt.init_state();
        let init_state: svm_byte_array = (INIT_STATE, init_state).into();

        // 4) execute app
        let func_name = "add_and_mul";
        let add = 5u32;
        let mul = 3u32;

        let mut calldata = svm_sdk::Vec::with_capacity(1000);

        add.encode(&mut calldata);
        mul.encode(&mut calldata);

        let exec_bytes = exec_app_bytes(&app_addr, func_name, &calldata);
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

        let receipt = receipt::decode_receipt(exec_receipt.clone().into()).into_exec_app();
        assert_eq!(receipt.success, true);

        let bytes = receipt.returndata();
        let mut returndata = ReturnData::new(bytes.as_slice());

        let [a, b, c]: [u32; 3] = returndata.next_1();

        assert_eq!(
            (a, b, c),
            (counter_init, counter_init + add, (counter_init + add) * mul)
        );

        assert_ne!(tracking::total_live(), 0);

        let _ = api::svm_byte_array_destroy(msg);
        let _ = api::svm_byte_array_destroy(app_bytes);
        let _ = api::svm_byte_array_destroy(exec_bytes);
        let _ = api::svm_byte_array_destroy(deployer);
        let _ = api::svm_byte_array_destroy(spawner);
        let _ = api::svm_byte_array_destroy(template_addr);
        let _ = api::svm_byte_array_destroy(app_addr);
        let _ = api::svm_byte_array_destroy(derived_app_addr);
        let _ = api::svm_byte_array_destroy(init_state);
        let _ = api::svm_byte_array_destroy(template_receipt);
        let _ = api::svm_byte_array_destroy(spawn_receipt);
        let _ = api::svm_byte_array_destroy(exec_receipt);
        let _ = api::svm_runtime_destroy(runtime);
        let _ = api::svm_state_kv_destroy(state_kv);

        assert_eq!(tracking::total_live(), 0);

        tracking::set_tracking_off();
    }
}
