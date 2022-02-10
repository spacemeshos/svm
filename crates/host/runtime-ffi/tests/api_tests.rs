extern crate svm_sdk_mock as svm_sdk;

use svm_codec::Codec;
use svm_layout::FixedLayout;
use svm_runtime_ffi as api;
use svm_runtime_testing as testing;
use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;
use svm_types::{Address, BytesPrimitive, Context, Envelope, Receipt, TemplateAddr};

use api::{svm_account, svm_init};

fn deploy_message(code_version: u32, name: &str, ctors: &[String], wasm: &[u8]) -> Vec<u8> {
    let layout = FixedLayout::from_byte_sizes(0, &[4]);

    testing::build_deploy(
        code_version,
        name,
        layout,
        ctors,
        testing::WasmFile::Binary(wasm),
    )
}

fn spawn_message(template_addr: &TemplateAddr, name: &str, ctor: &str, calldata: &[u8]) -> Vec<u8> {
    testing::build_spawn(template_addr, name, ctor, calldata)
}

fn call_message(target: &Address, func_name: &str, calldata: &[u8]) -> Vec<u8> {
    testing::build_call(&target, func_name, calldata)
}

#[test]
fn svm_runtime_success() {
    svm_init().unwrap();
    unsafe {
        // 1) `Init Runtime`
        let mut runtime = std::ptr::null_mut();
        api::svm_runtime_create(&mut runtime, std::ptr::null(), 0).unwrap();

        let res = api::svm_runtime_create(&mut runtime, std::ptr::null(), 0);
        assert!(res.is_ok());

        // 2) `Deploy Template`
        let deploy_msg = deploy_message(
            0,
            "My Template",
            &["initialize".to_string()],
            include_bytes!("wasm/counter.wasm"),
        );
        let principal = Address::repeat(0xAB);
        let deploy_env = Envelope::with_principal(principal).encode_to_vec();
        let deploy_ctx = Context::default().encode_to_vec();

        let res = api::svm_deploy(
            runtime,
            deploy_env.as_ptr(),
            deploy_msg.as_ptr(),
            deploy_msg.len() as u32,
            deploy_ctx.as_ptr(),
        );
        assert!(res.is_ok());

        // Extracts the deployed `Template Address`
        let receipt = Receipt::decode_bytes(res.receipt().unwrap())
            .unwrap()
            .into_deploy();
        let template_addr = receipt.template_addr();

        // 3) `Spawn Account`
        let mut calldata = svm_sdk::Vec::with_capacity(1000);
        Encoder::encode(&10u32, &mut calldata);

        let spawn_msg = spawn_message(&template_addr, "My Account", "initialize", &calldata);
        let spawner = Address::repeat(0xCD);
        let spawn_env = Envelope::with_principal(spawner).encode_to_vec();
        let spawn_ctx = Context::default().encode_to_vec();

        let res = api::svm_spawn(
            runtime,
            spawn_env.as_ptr(),
            spawn_msg.as_ptr(),
            spawn_msg.len() as u32,
            spawn_ctx.as_ptr(),
        );
        assert!(res.is_ok());

        let receipt = Receipt::decode_bytes(res.receipt().unwrap())
            .unwrap()
            .into_spawn();
        assert_eq!(receipt.success, true);

        // Extracts the Spawned `Account Address` and its initial `State`.
        let target = receipt.account_addr();
        let init_state = receipt.init_state();

        // 4) `Call Account`
        let mut calldata = svm_sdk::Vec::with_capacity(1000);
        Encoder::encode(&5u32, &mut calldata);

        let call_msg = call_message(&target, "add", &calldata);
        let principal = Address::repeat(0xEF);
        let call_env = Envelope::with_principal(principal).encode_to_vec();
        let call_ctx = Context::with_state(init_state.clone()).encode_to_vec();

        let res = api::svm_call(
            runtime,
            call_env.as_ptr(),
            call_msg.as_ptr(),
            call_msg.len() as u32,
            call_ctx.as_ptr(),
        );
        assert!(res.is_ok());

        let receipt = Receipt::decode_bytes(res.receipt().unwrap())
            .unwrap()
            .into_call();
        assert_eq!(receipt.success, true);

        let bytes = receipt.returndata();
        let mut returndata = ReturnData::new(bytes.as_slice());

        // Decodes the `Returns` back into native types.
        let [a, b]: [u32; 2] = returndata.next_1();
        assert_eq!((a, b), (10, 15));

        // Destroy `Runtime`
        api::svm_runtime_destroy(runtime).unwrap();
    }
}

#[test]
fn svm_runtime_failure() {
    svm_init().unwrap();
    unsafe {
        // 1) `Init Runtime`
        let mut runtime = std::ptr::null_mut();
        api::svm_runtime_create(&mut runtime, std::ptr::null(), 0).unwrap();

        // 2) `Deploy Template`
        let deploy_msg = deploy_message(
            0,
            "My Template",
            &["initialize".to_string()],
            include_bytes!("wasm/failure.wasm"),
        );
        let principal = Address::repeat(0xAB);
        let deploy_env = Envelope::with_principal(principal).encode_to_vec();
        let deploy_ctx = Context::default().encode_to_vec();

        let res = api::svm_deploy(
            runtime,
            deploy_env.as_ptr(),
            deploy_msg.as_ptr(),
            deploy_msg.len() as u32,
            deploy_ctx.as_ptr(),
        );
        assert!(res.is_ok());

        // Extracts the deployed `Template Address`
        let receipt = Receipt::decode_bytes(res.receipt().unwrap())
            .unwrap()
            .into_deploy();
        let template_addr = receipt.template_addr();

        // 3) `Spawn Account`
        let spawn_msg = spawn_message(&template_addr, "My Account", "initialize", &[]);
        let spawner = Address::repeat(0xCD);
        let spawn_env = Envelope::with_principal(spawner).encode_to_vec();
        let spawn_ctx = Context::default().encode_to_vec();

        let res = api::svm_spawn(
            runtime,
            spawn_env.as_ptr(),
            spawn_msg.as_ptr(),
            spawn_msg.len() as u32,
            spawn_ctx.as_ptr(),
        );
        assert!(res.is_ok());

        let receipt = Receipt::decode_bytes(res.receipt().unwrap())
            .unwrap()
            .into_spawn();
        assert_eq!(receipt.success, true);

        // Extracts the Spawned `Account Address` and its initial `State`.
        let target = receipt.account_addr();
        let init_state = receipt.init_state();

        // 4) `Call Account`
        let call_msg = call_message(&target, "fail", &[]);
        let principal = Address::repeat(0xEF);
        let call_env = Envelope::with_principal(principal).encode_to_vec();
        let call_ctx = Context::with_state(init_state.clone()).encode_to_vec();

        let res = api::svm_call(
            runtime,
            call_env.as_ptr(),
            call_msg.as_ptr(),
            call_msg.len() as u32,
            call_ctx.as_ptr(),
        );
        assert!(res.is_ok());

        let receipt = Receipt::decode_bytes(res.receipt().unwrap())
            .unwrap()
            .into_call();
        assert_eq!(receipt.success, false);

        // Destroy `Runtime`
        api::svm_runtime_destroy(runtime).unwrap();
    }
}

#[test]
fn svm_transfer_success() {
    api::svm_init().unwrap();
    unsafe {
        let src_addr = Address::repeat(0xAB);
        let dst_addr = Address::repeat(0xCD);

        let mut runtime = std::ptr::null_mut();

        api::svm_runtime_create(&mut runtime, std::ptr::null(), 0).unwrap();
        api::svm_create_genesis_account(runtime, src_addr.as_slice().as_ptr(), 1000, 0, 0).unwrap();
        api::svm_create_genesis_account(runtime, dst_addr.as_slice().as_ptr(), 0, 0, 0).unwrap();
        api::svm_transfer(
            runtime,
            src_addr.as_slice().as_ptr(),
            dst_addr.as_slice().as_ptr(),
            100,
        )
        .unwrap();

        let mut src = svm_account::default();
        let mut dst = svm_account::default();
        api::svm_get_account(runtime, src_addr.as_slice().as_ptr(), &mut src).unwrap();
        api::svm_get_account(runtime, dst_addr.as_slice().as_ptr(), &mut dst).unwrap();

        assert_eq!(src.balance, 900);
        assert_eq!(dst.balance, 100);

        api::svm_runtime_destroy(runtime).unwrap();
    }
}
