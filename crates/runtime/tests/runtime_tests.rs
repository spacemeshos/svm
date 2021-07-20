use svm_sdk as sdk;

use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;

use svm_codec::{Field, ParseError};
use svm_layout::FixedLayout;
use svm_program::ProgramError;
use svm_runtime::{testing, Runtime, ValidateError};

use svm_types::{Address, Context, Envelope, Gas, RuntimeError};
use svm_types::{DeployReceipt, SpawnReceipt};

#[test]
fn memory_runtime_validate_deploy_not_enough_bytes() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    let error = ParseError::NotEnoughBytes(Field::SectionKind);
    let expected = ValidateError::Parse(error);

    let actual = runtime.validate_deploy(&message).unwrap_err();
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_deploy_invalid_wasm() {
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
fn memory_runtime_validate_spawn_missing_template_addr() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    let error = ParseError::NotEnoughBytes(Field::Address);
    let expected = ValidateError::Parse(error);

    let actual = runtime.validate_spawn(&message).unwrap_err();
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_call_not_enough_bytes() {
    let runtime = testing::create_memory_runtime();
    let message = vec![0xFF, 0xFF];

    let error = ParseError::NotEnoughBytes(Field::AccountAddr);
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
    let name = "My Account";
    let ctor = "ctor";
    let calldata = vec![];
    let message = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let envelope = Envelope::with_gas_limit(Gas::with(0));

    let expected = SpawnReceipt::new_oog(Vec::new());
    let actual = runtime.spawn(&envelope, &message, &context);

    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_call_func_not_found() {
    let mut runtime = testing::create_memory_runtime();

    // 1) `Deploy Template`
    let code_version = 0;
    let layout: FixedLayout = vec![Address::len() as u32].into();
    let ctors = vec!["initialize".to_string()];
    let message = testing::build_deploy(
        code_version,
        "My Template",
        layout.clone(),
        &ctors,
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );
    let envelope = Envelope::default();
    let context = Context::default();

    let receipt = runtime.deploy(&envelope, &message, &context);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    let name = "My Account";
    let ctor = "initialize";
    let calldata = vec![];
    let message = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn(&envelope, &message, &context);
    assert!(receipt.success);

    let account_addr = receipt.account_addr();
    let init_state = receipt.init_state();
    let envelope = Envelope::default();
    let context = Context::with_state(init_state.clone());

    // 3) `Call Account`
    let message = testing::build_call(&account_addr, ctor, &[]);
    let receipt = runtime.call(&envelope, &message, &context);

    dbg!(receipt);

    // assert!(matches!(
    //     receipt.error.unwrap(),
    //     RuntimeError::FuncNotAllowed { .. }
    // ));
}

// #[test]
// fn memory_runtime_spawn_without_gas() {
//     let mut runtime = testing::create_memory_runtime();

//     // 1) `Deploy Template`
//     let code_version = 0;
//     let layout: FixedLayout = vec![Address::len() as u32].into();
//     let ctors = vec!["initialize".to_string()];

//     let bytes = testing::build_deploy(
//         code_version,
//         "My Template",
//         layout.clone(),
//         &ctors,
//         (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
//     );

//     let receipt = runtime.deploy(&envelope, &message, &context);
//     assert!(receipt.success);

//     let template_addr = receipt.addr.unwrap();

//     // 2) `Spawn Account`
//     let name = "My Account";
//     let ctor = "initialize";
//     let calldata = vec![];
//     let message = testing::build_spawn(&template_addr, name, ctor, &calldata);
//     let receipt = runtime.spawn(&envelope, &message, &context);

//     assert!(matches!(receipt.error.unwrap(), RuntimeError::OOG));
// }

// #[test]
// fn runtime_calldata_returndata() {
//     let mut runtime = testing::create_memory_runtime();

//     // 1) `Deploy Template`
//     let code_version = 0;
//     let layout: FixedLayout = vec![Address::len() as u32].into();
//     let ctors = vec!["initialize".to_string()];

//     let bytes = testing::build_deploy(
//         code_version,
//         "My Template",
//         layout.clone(),
//         &ctors,
//         (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
//     );

//     let receipt = runtime.deploy(&envelope, &message, &context);
//     assert!(receipt.success);

//     let template_addr = receipt.addr.unwrap();

//     // 2) `Spawn Account`
//     let name = "My Account";
//     let ctor = "initialize";
//     let calldata = vec![];
//     let message = testing::build_spawn(&template_addr, name, ctor, &calldata);
//     let receipt = runtime.spawn(&envelope, &message, &context);
//     assert!(receipt.success);

//     let account_addr = receipt.account_addr();
//     let init_state = receipt.init_state();

//     // 3) `Call Account`
//     let func = "store_addr";
//     let msg: sdk::Address = sdk::Address::repeat(0x10);

//     let mut calldata = svm_sdk::Vec::with_capacity(100_000);
//     msg.encode(&mut calldata);

//     let message = testing::build_call(&account_addr, func, &calldata);
//     let res = runtime.validate_call(&message).unwrap();
//     assert!(res.is_ok());

//     let receipt = runtime.call(&envelope, &message, &context);
//     assert!(receipt.success);

//     let state = receipt.new_state();

//     // 4) `Call Account` (calling a function with returns)
//     let func = "load_addr";
//     let calldata = Vec::new();

//     let message = testing::build_call(&account_addr, func, &calldata);
//     let receipt = runtime.call(&envelope, &message, &context);
//     assert!(receipt.success);

//     let bytes = receipt.returndata.unwrap();
//     let mut returndata = ReturnData::new(&bytes);

//     let addr: sdk::Address = returndata.next_1();
//     assert_eq!(addr.as_slice(), &[0x10; 20]);
// }
