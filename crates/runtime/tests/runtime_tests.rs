use svm_sdk as sdk;

use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;

use svm_codec::{Field, ParseError};
use svm_layout::FixedLayout;
use svm_program::ProgramError;
use svm_runtime::{testing, Runtime, ValidateError};

use svm_types::{Address, Gas, RuntimeError};
use svm_types::{DeployReceipt, SpawnReceipt};

fn memory_runtime() -> impl Runtime {
    let state_kv = testing::memory_kv_init();
    testing::create_memory_runtime(&state_kv)
}

#[test]
#[ignore]
fn memory_runtime_validate_deploy_missing_name() {
    let runtime = memory_runtime();
    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::Name);
    let expected = ValidateError::Parse(parse_err);

    let actual = runtime.validate_deploy(&bytes[..]).unwrap_err();
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_deploy_invalid_wasm() {
    let runtime = memory_runtime();
    let code_version = 0;
    let ctors = Vec::new();

    // An invalid Wasm (has floats)
    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        FixedLayout::default(),
        &ctors,
        include_str!("wasm/wasm_with_floats.wast").into(),
    );

    let prog_err = ProgramError::FloatsNotAllowed;
    let expected = Err(ValidateError::Program(prog_err));

    let actual = runtime.validate_deploy(&bytes[..]);
    assert_eq!(expected, actual);
}

#[ignore]
#[test]
fn memory_runtime_validate_spawn_missing_template_addr() {
    let runtime = memory_runtime();
    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::TemplateAddr);
    let expected = ValidateError::Parse(parse_err);

    let actual = runtime.validate_spawn(&bytes).unwrap_err();
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_validate_call_missing_target() {
    let runtime = memory_runtime();

    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::AccountAddr);
    let expected = Err(ValidateError::Parse(parse_err));

    let actual = runtime.validate_call(&bytes);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_deploy_reaches_oog() {
    let mut runtime = memory_runtime();

    let code_version = 0;
    let deployer = Address::of("deployer").into();
    let maybe_gas = Gas::with(0);
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        FixedLayout::default(),
        &ctors,
        include_str!("wasm/runtime_spawn.wast").into(),
    );

    let expected = DeployReceipt::new_oog();
    let actual = runtime.deploy(&bytes, &deployer, maybe_gas);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_deploy_success() {
    let mut runtime = memory_runtime();

    let code_version = 0;
    let deployer = Address::of("deployer").into();
    let gas_limit = Gas::with(1_0000_000);
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        FixedLayout::default(),
        &ctors,
        include_str!("wasm/runtime_spawn.wast").into(),
    );

    let receipt = runtime.deploy(&bytes, &deployer, gas_limit);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());
}

#[test]
fn memory_runtime_spawn_invoking_non_ctor_fails() {
    let mut runtime = memory_runtime();

    // 1) `Deploy Template`
    let code_version = 0;
    let deployer = Address::of("deployer").into();
    let creator = Address::of("creator").into();
    let maybe_gas = Gas::new();
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        FixedLayout::default(),
        &ctors,
        include_str!("wasm/runtime_spawn.wast").into(),
    );

    let receipt = runtime.deploy(&bytes, &deployer, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account` (and trying to use a non-`ctor` function as a `ctor`)
    let name = "My Account";
    let ctor = "non_ctor";
    let calldata = vec![];

    let bytes = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let maybe_gas = Gas::new();

    let receipt = runtime.spawn(&bytes, &creator, maybe_gas);
    assert!(matches!(
        receipt.error.unwrap(),
        RuntimeError::FuncNotAllowed { .. }
    ));
}

#[test]
fn memory_runtime_spawn_reaches_oog() {
    let mut runtime = memory_runtime();

    // 1) `Deploy Template`
    let code_version = 0;
    let deployer = Address::of("deployer").into();
    let creator = Address::of("creator").into();
    let maybe_gas = Gas::new();
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        FixedLayout::default(),
        &ctors,
        include_str!("wasm/runtime_spawn.wast").into(),
    );

    let receipt = runtime.deploy(&bytes, &deployer, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    let name = "My Account";
    let ctor = "ctor";
    let calldata = vec![];

    let bytes = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let maybe_gas = Gas::with(0);

    let expected = SpawnReceipt::new_oog(Vec::new());
    let actual = runtime.spawn(&bytes, &creator, maybe_gas);
    assert_eq!(expected, actual);
}

#[test]
fn memory_runtime_call_func_not_found() {
    let mut runtime = memory_runtime();

    // 1) `Deploy Template`
    let code_version = 0;
    let deployer = Address::of("deployer").into();
    let maybe_gas = Gas::new();
    let layout: FixedLayout = vec![Address::len() as u32].into();
    let ctors = vec!["initialize".to_string()];

    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        layout.clone(),
        &ctors,
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );

    let receipt = runtime.deploy(&bytes, &deployer, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    let name = "My Account";
    let ctor = "initialize";
    let calldata = vec![];
    let creator = Address::of("creator").into();
    let bytes = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn(&bytes, &creator, maybe_gas);
    assert!(receipt.success);

    let account_addr = receipt.account_addr();
    let init_state = receipt.init_state();

    // 3) `Call Account`
    let calldata = Vec::new();
    let bytes = testing::build_call(&account_addr, ctor, &calldata);
    let tx = runtime.validate_call(&bytes).unwrap();

    let receipt = runtime.call(&tx, &init_state, maybe_gas);

    assert!(matches!(
        receipt.error.unwrap(),
        RuntimeError::FuncNotAllowed { .. }
    ));
}

#[test]
fn memory_runtime_spawn_without_gas() {
    let mut runtime = memory_runtime();

    // 1) `Deploy Template`
    let code_version = 0;
    let deployer = Address::of("deployer").into();
    let layout: FixedLayout = vec![Address::len() as u32].into();
    let ctors = vec!["initialize".to_string()];

    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        layout.clone(),
        &ctors,
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );

    let receipt = runtime.deploy(&bytes, &deployer, Gas::new());
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    let name = "My Account";
    let ctor = "initialize";
    let calldata = vec![];
    let creator = Address::of("creator").into();
    let bytes = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn(&bytes, &creator, Gas::with(0));

    assert!(matches!(receipt.error.unwrap(), RuntimeError::OOG));
}

#[test]
fn runtime_calldata_returndata() {
    let mut runtime = memory_runtime();

    // 1) `Deploy Template`
    let code_version = 0;
    let deployer = Address::of("deployer").into();
    let maybe_gas = Gas::new();
    let layout: FixedLayout = vec![Address::len() as u32].into();
    let ctors = vec!["initialize".to_string()];

    let bytes = testing::build_deploy(
        code_version,
        "My Template",
        layout.clone(),
        &ctors,
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );

    let receipt = runtime.deploy(&bytes, &deployer, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) `Spawn Account`
    let name = "My Account";
    let ctor = "initialize";
    let calldata = vec![];
    let creator = Address::of("creator").into();
    let bytes = testing::build_spawn(&template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn(&bytes, &creator, maybe_gas);
    assert!(receipt.success);

    let account_addr = receipt.account_addr();
    let init_state = receipt.init_state();

    // 3) `Call Account`
    let func = "store_addr";
    let msg: sdk::Address = sdk::Address::repeat(0x10);

    let mut calldata = svm_sdk::Vec::with_capacity(100_000);
    msg.encode(&mut calldata);

    let bytes = testing::build_call(&account_addr, func, &calldata);
    let tx = runtime.validate_call(&bytes).unwrap();

    let receipt = runtime.call(&tx, &init_state, maybe_gas);
    assert!(receipt.success);

    let state = receipt.new_state();

    // 4) `Call Account` (calling a function with returns)
    let func = "load_addr";
    let calldata = Vec::new();

    let bytes = testing::build_call(&account_addr, func, &calldata);
    let tx = runtime.validate_call(&bytes).unwrap();

    let receipt = runtime.call(&tx, &state, maybe_gas);
    assert!(receipt.success);

    let bytes = receipt.returndata.unwrap();
    let mut returndata = ReturnData::new(&bytes);

    let addr: sdk::Address = returndata.next_1();
    assert_eq!(addr.as_slice(), &[0x10; 20]);
}
