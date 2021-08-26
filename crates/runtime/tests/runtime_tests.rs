use svm_sdk as sdk;

use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;

use svm_codec::{Field, ParseError};
use svm_layout::FixedLayout;
use svm_program::ProgramError;
use svm_runtime::{testing, Runtime, ValidateError};

use svm_types::{
    Address, BytesPrimitive, Context, DeployReceipt, Envelope, Gas, RuntimeError, SpawnReceipt,
};

#[test]
fn memory_runtime_validate_deploy_not_enough_bytes() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    let error = ParseError::Eof(Field::SectionKind.to_string());
    let expected = ValidateError::Parse(error);

    let actual = runtime.validate_deploy(&message).unwrap_err();
    assert_eq!(expected, actual);
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
fn memory_runtime_validate_spawn_missing_template_addr() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    let error = ParseError::Eof(Field::Address.to_string());
    let expected = ValidateError::Parse(error);

    let actual = runtime.validate_spawn(&message).unwrap_err();
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_call_not_enough_bytes() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    let error = ParseError::Eof(Field::TargetAddr.to_string());
    let expected = Err(ValidateError::Parse(error));

    let actual = runtime.validate_call(&message);
    assert_eq!(expected, actual);
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
        RuntimeError::FuncNotAllowed { .. }
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
    let layout: FixedLayout = vec![Address::len() as u32].into();
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
    let message = testing::build_spawn(&template_addr, "My Account", "initialize", &[]);
    let receipt = runtime.spawn(&envelope, &message, &context);
    assert!(receipt.success);

    let spawned_addr = receipt.account_addr();
    let init_state = receipt.init_state();
    let envelope = Envelope::default();
    let context = Context::with_state(init_state.clone());

    // 3) `Call Account`
    let message = testing::build_call(&spawned_addr, "initialize", &[]);
    let receipt = runtime.call(&envelope, &message, &context);

    assert!(matches!(
        receipt.error.unwrap(),
        RuntimeError::FuncNotAllowed { .. }
    ));
}

#[test]
fn memory_runtime_call_success() {
    let mut runtime = testing::create_memory_runtime();

    // 1) `Deploy Template`
    let layout: FixedLayout = vec![Address::len() as u32].into();
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
    let message = testing::build_spawn(&template_addr, "My Account", "initialize", &[]);
    let receipt = runtime.spawn(&envelope, &message, &context);
    assert!(receipt.success);

    let spawned_addr = receipt.account_addr();
    let init_state = receipt.init_state();

    // 3) `Call Account`

    // Preparing the binary `CallData`
    // Encoding the `Address = "10 10 ... 10"`
    let param: sdk::Address = sdk::Address::repeat(0x10);
    let mut calldata = svm_sdk::Vec::with_capacity(Address::len() + 1);
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
