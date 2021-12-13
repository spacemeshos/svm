extern crate svm_sdk_mock as svm_sdk;

use svm_codec::ParseError;
use svm_layout::FixedLayout;
use svm_program::ProgramError;
use svm_runtime::{compute_account_addr, testing, ValidateError};
use svm_sdk as sdk;
use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;

use svm_types::{
    Address, BytesPrimitive, Context, DeployReceipt, Envelope, Gas, RuntimeError, SpawnAccount,
    SpawnReceipt,
};

#[test]
fn memory_runtime_validate_deploy_eof() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    assert!(matches!(
        runtime.validate_deploy(&message),
        Err(ValidateError::Parse(ParseError::Eof))
    ));
}

#[test]
fn memory_runtime_validate_deploy_missing_svm_verify_export() {
    let runtime = testing::create_memory_runtime();

    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &[],
        include_str!("wasm/missing_svm_verify.wast").into(),
    );

    let error = ProgramError::FunctionNotFound("svm_verify".to_string());
    let expected = Err(ValidateError::Program(error));

    let actual = runtime.validate_deploy(&message);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_deploy_missing_svm_alloc_export() {
    let runtime = testing::create_memory_runtime();

    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &[],
        include_str!("wasm/missing_svm_alloc.wast").into(),
    );

    let error = ProgramError::FunctionNotFound("svm_alloc".to_string());
    let expected = Err(ValidateError::Program(error));

    let actual = runtime.validate_deploy(&message);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_deploy_svm_alloc_export_invalid_signature() {
    let runtime = testing::create_memory_runtime();

    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &[],
        include_str!("wasm/svm_alloc_invalid_sig.wast").into(),
    );

    let error = ProgramError::InvalidExportFunctionSignature("svm_alloc".to_string());
    let expected = Err(ValidateError::Program(error));

    let actual = runtime.validate_deploy(&message);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_deploy_svm_verify_export_invalid_signature() {
    let runtime = testing::create_memory_runtime();

    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &[],
        include_str!("wasm/svm_verify_invalid_sig.wast").into(),
    );

    let error = ProgramError::InvalidExportFunctionSignature("svm_verify".to_string());
    let expected = Err(ValidateError::Program(error));

    let actual = runtime.validate_deploy(&message);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_deploy_floats_not_allowed() {
    let runtime = testing::create_memory_runtime();

    // An invalid Wasm (has floats)
    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &[],
        include_str!("wasm/wasm_with_floats.wast").into(),
    );

    let error = ProgramError::FloatsNotAllowed;
    let expected = Err(ValidateError::Program(error));

    let actual = runtime.validate_deploy(&message);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_deploy_ok() {
    let runtime = testing::create_memory_runtime();

    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &[],
        include_bytes!("wasm/runtime_calldata.wasm")[..].into(),
    );

    let result = runtime.validate_deploy(&message);
    assert!(result.is_ok());
}

#[test]
fn memory_runtime_validate_spawn_eof() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    assert!(matches!(
        runtime.validate_spawn(&message),
        Err(ValidateError::Parse(ParseError::Eof))
    ));
}

#[test]
fn memory_runtime_validate_call_eof() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    assert!(matches!(
        runtime.validate_call(&message),
        Err(ValidateError::Parse(ParseError::Eof))
    ));
}

#[test]
fn memory_runtime_deploy_reaches_oog() {
    let mut runtime = testing::create_memory_runtime();

    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &["ctor".to_string()],
        include_str!("wasm/runtime_spawn.wast").into(),
    );
    let envelope = Envelope::with_gas_limit(Gas::with(0));
    let context = Context::default();

    let expected = DeployReceipt::new_oog();
    let actual = runtime.deploy(&envelope, &message, &context);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_deploy_success() {
    let mut runtime = testing::create_memory_runtime();

    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &["ctor".to_string()],
        include_str!("wasm/runtime_spawn.wast").into(),
    );
    let envelope = Envelope::default();
    let context = Context::default();

    let receipt = runtime.deploy(&envelope, &message, &context);

    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());
}

#[test]
fn memory_runtime_spawn_invoking_non_ctor_fails() {
    let mut runtime = testing::create_memory_runtime();
    let envelope = Envelope::default();
    let context = Context::default();

    // 1) `Deploy Template`
    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &["ctor".to_string()],
        include_str!("wasm/runtime_spawn.wast").into(),
    );

    let receipt = runtime.deploy(&envelope, &message, &context);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account` (and trying to use a non-`ctor` function as a `ctor`)
    let name = "My Account";
    let ctor = "non_ctor";
    let calldata = vec![];

    let message = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn(&envelope, &message, &context);

    assert!(matches!(
        receipt.error.unwrap(),
        RuntimeError::FuncNotCtor { .. }
    ));
}

#[test]
fn memory_runtime_spawn_reaches_oog() {
    let mut runtime = testing::create_memory_runtime();

    // 1) `Deploy Template`
    let message = testing::build_deploy(
        0,
        "My Template",
        FixedLayout::default(),
        &["ctor".to_string()],
        include_str!("wasm/runtime_spawn.wast").into(),
    );
    let envelope = Envelope::default();
    let context = Context::default();

    let receipt = runtime.deploy(&envelope, &message, &context);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    let message = testing::build_spawn(&template_addr, "My Account", "ctor", &[]);
    let envelope = Envelope::with_gas_limit(Gas::with(0));

    let expected = SpawnReceipt::new_oog(Vec::new());
    let actual = runtime.spawn(&envelope, &message, &context);

    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_call_func_not_found() {
    let mut runtime = testing::create_memory_runtime();

    // 1) `Deploy Template`
    let layout = FixedLayout::from_byte_sizes(0, &[Address::N as u32]);
    let message = testing::build_deploy(
        0,
        "My Template",
        layout.clone(),
        &["initialize".to_string()],
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );
    let envelope = Envelope::default();
    let context = Context::default();

    let receipt = runtime.deploy(&envelope, &message, &context);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    // Prepare the `calldata`
    let param: sdk::Address = sdk::Address::repeat(0x10);
    let mut calldata = svm_sdk::Vec::with_capacity(Address::N + 2);
    param.encode(&mut calldata);
    true.encode(&mut calldata);

    let message = testing::build_spawn(&template_addr, "My Account", "initialize", &calldata);
    let receipt = runtime.spawn(&envelope, &message, &context);
    assert!(receipt.success);

    let spawned_addr = receipt.account_addr();
    let init_state = receipt.init_state();
    let envelope = Envelope::default();
    let context = Context::with_state(init_state.clone());

    // 3) `Call Account`
    let message = testing::build_call(&spawned_addr, "no_such_func", &[]);
    let receipt = runtime.call(&envelope, &message, &context);

    assert!(matches!(
        receipt.error.unwrap(),
        RuntimeError::FuncNotFound { .. }
    ));
}

#[test]
fn memory_runtime_call_success() {
    let mut runtime = testing::create_memory_runtime();

    // 1) `Deploy Template`
    let layout = FixedLayout::from_byte_sizes(0, &[Address::N as u32]);
    let message = testing::build_deploy(
        0,
        "My Template",
        layout.clone(),
        &["initialize".to_string()],
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );
    let envelope = Envelope::default();
    let context = Context::default();
    let receipt = runtime.deploy(&envelope, &message, &context);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    // Prepare the `calldata`
    let param: sdk::Address = sdk::Address::repeat(0x10);
    let mut calldata = svm_sdk::Vec::with_capacity(Address::N + 1 + 1);
    param.encode(&mut calldata);
    true.encode(&mut calldata);

    let message = testing::build_spawn(&template_addr, "My Account", "initialize", &calldata);
    let receipt = runtime.spawn(&envelope, &message, &context);
    assert!(receipt.success);

    let spawned_addr = receipt.account_addr().clone();
    let init_state = receipt.init_state().clone();

    let bytes = receipt.returndata.unwrap();
    let mut returndata = ReturnData::new(&bytes);

    let status: bool = returndata.next_1();
    assert_eq!(status, true);

    // 3) `Call Account`
    // Preparing the binary `CallData`
    // Encoding the `Address = "10 10 ... 10"`
    let param: sdk::Address = sdk::Address::repeat(0x10);
    let mut calldata = svm_sdk::Vec::with_capacity(Address::N + 1);
    param.encode(&mut calldata);

    let message = testing::build_call(&spawned_addr, "store_addr", &calldata);
    let envelope = Envelope::default();
    let context = Context::with_state(init_state.clone());

    let receipt = runtime.call(&envelope, &message, &context);
    assert!(receipt.success);

    let new_state = receipt.new_state();

    // 4) `Call Account` (calling a function this with `returns` this time)
    let message = testing::build_call(&spawned_addr, "load_addr", &[]);
    let envelope = Envelope::default();
    let context = Context::with_state(new_state.clone());

    let receipt = runtime.call(&envelope, &message, &context);
    assert!(receipt.success);

    let bytes = receipt.returndata.unwrap();
    let mut returndata = ReturnData::new(&bytes);

    let addr: sdk::Address = returndata.next_1();
    assert_eq!(addr.as_slice(), &[0x10; 20]);
}

#[test]
fn spawn_touched_accounts() {
    let mut runtime = testing::create_memory_runtime();

    // 1) `Deploy Template`
    let layout = FixedLayout::from_byte_sizes(0, &[Address::N as u32]);
    let message = testing::build_deploy(
        0,
        "My Template",
        layout.clone(),
        &["initialize".to_string()],
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );
    let envelope = Envelope::default();
    let context = Context::default();
    let receipt = runtime.deploy(&envelope, &message, &context);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    // Prepare the `calldata`
    let param: sdk::Address = sdk::Address::repeat(0x10);
    let mut calldata = svm_sdk::Vec::with_capacity(Address::N + 1 + 1);
    param.encode(&mut calldata);
    true.encode(&mut calldata);

    let spawn = SpawnAccount::new(0, &template_addr, "My Account", "initialize", &calldata[..]);
    let message = svm_codec::Codec::encode_to_vec(&spawn);
    let receipt = runtime.spawn(&envelope, &message, &context);

    assert!(receipt.success);
    assert_eq!(receipt.touched_accounts.len(), 2);
    assert!(receipt.touched_accounts.contains(envelope.principal()));
    assert!(receipt
        .touched_accounts
        .contains(&compute_account_addr(&spawn)));
}
