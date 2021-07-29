use maplit::hashmap;

use std::collections::HashMap;
use std::convert::TryFrom;

use svm_runtime_ffi as api;
use svm_runtime_ffi::{svm_byte_array, tracking};

use svm_codec::receipt;
use svm_runtime::testing;
use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;
use svm_types::{Address, Context, Envelope, TemplateAddr, Type};

unsafe fn byte_array_copy(byte_array: &svm_byte_array, bytes: &[u8]) {
    debug_assert_eq!(byte_array.len() as usize, bytes.len());

    let src = bytes.as_ptr();
    let dst = byte_array.bytes_mut();

    std::ptr::copy(src, dst, bytes.len());
}

fn deploy_message(code_version: u32, name: &str, ctors: &[String], wasm: &[u8]) -> svm_byte_array {
    use svm_layout::{FixedLayoutBuilder, Id};

    let mut builder = FixedLayoutBuilder::default();
    builder.set_first(Id(0));
    builder.push(4);
    let layout = builder.build();

    let msg = testing::build_deploy(
        code_version,
        name,
        layout,
        ctors,
        testing::WasmFile::Binary(wasm),
    );

    let byte_array = unsafe { api::svm_message_alloc(msg.len() as u32) };
    unsafe { byte_array_copy(&byte_array, &msg) };

    byte_array
}

fn spawn_message(
    template_addr: &TemplateAddr,
    name: &str,
    ctor: &str,
    calldata: &[u8],
) -> svm_byte_array {
    let msg = testing::build_spawn(template_addr, name, ctor, calldata);

    let byte_array = unsafe { api::svm_message_alloc(msg.len() as u32) };
    unsafe { byte_array_copy(&byte_array, &msg) };

    byte_array
}

fn call_message(target: &Address, func_name: &str, calldata: &[u8]) -> svm_byte_array {
    let msg = testing::build_call(&target, func_name, calldata);

    let byte_array = unsafe { api::svm_message_alloc(msg.len() as u32) };
    unsafe { byte_array_copy(&byte_array, &msg) };

    byte_array
}

fn encode_envelope(env: &Envelope) -> svm_byte_array {
    use svm_codec::envelope;

    let byte_array = unsafe { api::svm_envelope_alloc() };

    let mut bytes = Vec::new();
    envelope::encode(env, &mut bytes);

    unsafe { byte_array_copy(&byte_array, &bytes) };

    byte_array
}

fn encode_context(ctx: &Context) -> svm_byte_array {
    use svm_codec::context;

    let byte_array = unsafe { api::svm_context_alloc() };

    let mut bytes = Vec::new();
    context::encode(ctx, &mut bytes);

    unsafe { byte_array_copy(&byte_array, &bytes) };

    byte_array
}

unsafe fn destroy(byte_arrays: &[svm_byte_array]) {
    for byte_array in byte_arrays {
        let _ = api::svm_byte_array_destroy(byte_array.clone());
    }
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

        let _ = api::svm_resource_type_name_destroy(raw_ty1);
        let _ = api::svm_resource_type_name_destroy(raw_ty2);

        let _ = api::svm_resource_destroy(r1);
        let _ = api::svm_resource_destroy(r2);
        let _ = api::svm_resource_iter_destroy(iter);

        let _ = destroy(&[hello, world, new_world]);

        assert_eq!(api::svm_total_live_resources(), 0);

        tracking::set_tracking_off();
    }
}

#[test]
fn svm_runtime_success() {
    unsafe {
        tracking::set_tracking_on();

        assert_eq!(tracking::total_live(), 0);

        // 1) `Init Runtime`
        let mut runtime = std::ptr::null_mut();
        let mut error = svm_byte_array::default();

        let res = api::svm_memory_runtime_create(&mut runtime, &mut error);
        assert!(res.is_ok());

        // 2) `Deploy Template`
        let deploy_msg = deploy_message(
            0,
            "My Template",
            &["initialize".to_string()],
            include_bytes!("wasm/counter.wasm"),
        );
        let principal = Address::repeat(0xAB);
        let deploy_env = encode_envelope(&Envelope::with_principal(principal));
        let deploy_ctx = encode_context(&Context::default());

        let mut deploy_receipt = svm_byte_array::default();
        let res = api::svm_deploy(
            &mut deploy_receipt,
            runtime,
            deploy_env.clone(),
            deploy_msg.clone(),
            deploy_ctx.clone(),
            &mut error,
        );
        assert!(res.is_ok());

        // Extracts the deployed `Template Address`
        let receipt = receipt::decode_receipt(deploy_receipt.as_slice()).into_deploy();
        let template_addr = receipt.template_addr();

        // 3) `Spawn Account`
        let mut calldata = svm_sdk::Vec::with_capacity(1000);
        10u32.encode(&mut calldata);

        let spawn_msg = spawn_message(&template_addr, "My Account", "initialize", &calldata);
        let spawner = Address::repeat(0xCD);
        let spawn_env = encode_envelope(&Envelope::with_principal(spawner));
        let spawn_ctx = encode_context(&Context::default());

        let mut spawn_receipt = svm_byte_array::default();
        let res = api::svm_spawn(
            &mut spawn_receipt,
            runtime,
            spawn_env.clone(),
            spawn_msg.clone(),
            spawn_ctx.clone(),
            &mut error,
        );
        assert!(res.is_ok());

        let receipt = receipt::decode_receipt(spawn_receipt.as_slice()).into_spawn();
        assert_eq!(receipt.success, true);

        // Extracts the Spawned `Account Address` and its initial `State`.
        let target = receipt.account_addr();
        let init_state = receipt.init_state();

        // 4) `Call Account`
        let mut calldata = svm_sdk::Vec::with_capacity(1000);
        5u32.encode(&mut calldata);

        let call_msg = call_message(&target, "add", &calldata);
        let principal = Address::repeat(0xEF);
        let call_env = encode_envelope(&Envelope::with_principal(principal));
        let call_ctx = encode_context(&Context::with_state(init_state.clone()));

        let mut call_receipt = svm_byte_array::default();
        let res = api::svm_call(
            &mut call_receipt,
            runtime,
            call_env.clone(),
            call_msg.clone(),
            call_ctx.clone(),
            &mut error,
        );
        assert!(res.is_ok());

        let receipt = receipt::decode_receipt(call_receipt.as_slice()).into_call();
        assert_eq!(receipt.success, true);

        let bytes = receipt.returndata();
        let mut returndata = ReturnData::new(bytes.as_slice());

        // Decodes the `Returns` back into native types.
        let [a, b]: [u32; 2] = returndata.next_1();
        assert_eq!((a, b), (10, 15));

        // Asserts there are resources to be destroyed.
        assert_ne!(tracking::total_live(), 0);

        // Destroy `svm_byte_array`s
        destroy(&[deploy_env, spawn_env, call_env]);
        destroy(&[deploy_msg, spawn_msg, call_msg]);
        destroy(&[deploy_ctx, spawn_ctx, call_ctx]);
        destroy(&[deploy_receipt, spawn_receipt, call_receipt]);

        // Destroy `Runtime`
        let _ = api::svm_runtime_destroy(runtime);

        // Asserts there are NO leaked resources
        assert_eq!(tracking::total_live(), 0);

        tracking::set_tracking_off();
    }
}

//
#[test]
fn svm_runtime_failure() {
    unsafe {
        tracking::set_tracking_on();

        // Asserting we start the test with no previous leakage.
        assert_eq!(tracking::total_live(), 0);

        // 1) `Init Runtime`
        let mut runtime = std::ptr::null_mut();
        let mut error = svm_byte_array::default();

        let res = api::svm_memory_runtime_create(&mut runtime, &mut error);
        assert!(res.is_ok());

        // 2) `Deploy Template`
        let deploy_msg = deploy_message(
            0,
            "My Template",
            &["initialize".to_string()],
            include_bytes!("wasm/failure.wasm"),
        );
        let principal = Address::repeat(0xAB);
        let deploy_env = encode_envelope(&Envelope::with_principal(principal));
        let deploy_ctx = encode_context(&Context::default());

        let mut deploy_receipt = svm_byte_array::default();
        let res = api::svm_deploy(
            &mut deploy_receipt,
            runtime,
            deploy_env.clone(),
            deploy_msg.clone(),
            deploy_ctx.clone(),
            &mut error,
        );
        assert!(res.is_ok());

        // Extracts the deployed `Template Address`
        let receipt = receipt::decode_receipt(deploy_receipt.as_slice()).into_deploy();
        let template_addr = receipt.template_addr();

        // 3) `Spawn Account`
        let spawn_msg = spawn_message(&template_addr, "My Account", "initialize", &[]);
        let spawner = Address::repeat(0xCD);
        let spawn_env = encode_envelope(&Envelope::with_principal(spawner));
        let spawn_ctx = encode_context(&Context::default());

        let mut spawn_receipt = svm_byte_array::default();
        let res = api::svm_spawn(
            &mut spawn_receipt,
            runtime,
            spawn_env.clone(),
            spawn_msg.clone(),
            spawn_ctx.clone(),
            &mut error,
        );
        assert!(res.is_ok());

        let receipt = receipt::decode_receipt(spawn_receipt.as_slice()).into_spawn();
        assert_eq!(receipt.success, true);

        // Extracts the Spawned `Account Address` and its initial `State`.
        let target = receipt.account_addr();
        let init_state = receipt.init_state();

        // 4) `Call Account`
        let call_msg = call_message(&target, "fail", &[]);
        let principal = Address::repeat(0xEF);
        let call_env = encode_envelope(&Envelope::with_principal(principal));
        let call_ctx = encode_context(&Context::with_state(init_state.clone()));

        let mut call_receipt = svm_byte_array::default();
        let res = api::svm_call(
            &mut call_receipt,
            runtime,
            call_env.clone(),
            call_msg.clone(),
            call_ctx.clone(),
            &mut error,
        );
        assert!(res.is_ok());

        let receipt = receipt::decode_receipt(call_receipt.as_slice()).into_call();
        assert_eq!(receipt.success, false);

        // Asserts there are resources to be destroyed.
        assert_ne!(tracking::total_live(), 0);

        // Destroy `svm_byte_array`s
        destroy(&[deploy_env, spawn_env, call_env]);
        destroy(&[deploy_msg, spawn_msg, call_msg]);
        destroy(&[deploy_ctx, spawn_ctx, call_ctx]);
        destroy(&[deploy_receipt, spawn_receipt, call_receipt]);

        // Destroy `Runtime`
        let _ = api::svm_runtime_destroy(runtime);

        // Asserts there are NO leaked resources
        assert_eq!(tracking::total_live(), 0);

        tracking::set_tracking_off();
    }
}
