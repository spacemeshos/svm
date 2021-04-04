use svm_sdk as sdk;

use svm_sdk::traits::Encoder;
use svm_sdk::ReturnData;

use svm_codec::{Field, ParseError};

use svm_gas::error::ProgramError;
use svm_layout::Layout;
use svm_runtime::{error::ValidateError, testing, Runtime};

use svm_types::{Address, Gas, RuntimeError};
use svm_types::{SpawnAppReceipt, TemplateReceipt};

macro_rules! default_runtime {
    () => {{
        use svm_runtime::testing;

        let state_kv = testing::memory_state_kv_init();
        let imports = Vec::new();

        let imports = Box::leak(Box::new(imports));

        testing::create_memory_runtime(&state_kv, imports)
    }};
}

#[ignore]
#[test]
fn default_runtime_validate_template_invalid_raw_format() {
    let runtime = default_runtime!();
    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::Name);
    let expected = ValidateError::Parse(parse_err);

    let actual = runtime.validate_template(&bytes[..]).unwrap_err();
    assert_eq!(expected, actual);
}

#[ignore = "temporarily disabling this test"]
#[test]
fn default_runtime_validate_template_invalid_wasm() {
    let runtime = default_runtime!();
    let version = 0;
    let ctors = Vec::new();

    // invalid wasm (has floats)
    let bytes = testing::build_template(
        version,
        "My Template",
        Layout::empty(),
        &ctors,
        include_str!("wasm/wasm_with_floats.wast").into(),
    );

    let prog_err = ProgramError::FloatsNotAllowed;
    let expected = Err(ValidateError::Program(prog_err));

    let actual = runtime.validate_template(&bytes[..]);
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
fn default_runtime_validate_app_invalid_raw_format() {
    let runtime = default_runtime!();
    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::TemplateAddr);
    let expected = ValidateError::Parse(parse_err);

    let actual = runtime.validate_app(&bytes).unwrap_err();
    assert_eq!(expected, actual);
}

#[ignore = "temporarily disabling this test"]
#[test]
fn default_runtime_validate_tx_invalid_raw_format() {
    let runtime = default_runtime!();

    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::AppAddr);
    let expected = Err(ValidateError::Parse(parse_err));

    let actual = runtime.validate_tx(&bytes);
    assert_eq!(expected, actual);
}

#[ignore = "temporarily disabling this test"]
#[test]
fn default_runtime_deploy_template_reaches_oog() {
    let mut runtime = default_runtime!();

    let version = 0;
    let author = Address::of("author").into();
    let maybe_gas = Gas::with(0);
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        Layout::empty(),
        &ctors,
        include_str!("wasm/runtime_app_ctor.wast").into(),
    );

    let expected = TemplateReceipt::new_oog();
    let actual = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert_eq!(expected, actual);
}

#[ignore = "temporarily disabling this test"]
#[test]
fn default_runtime_deploy_template_has_enough_gas() {
    let mut runtime = default_runtime!();

    let version = 0;
    let author = Address::of("author").into();
    let gas_limit = Gas::with(1_0000_000);
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        Layout::empty(),
        &ctors,
        include_str!("wasm/runtime_app_ctor.wast").into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, gas_limit);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());
}

#[ignore = "temporarily disabling this test"]
#[test]
fn default_runtime_spawn_app_with_non_ctor_fails() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();
    let maybe_gas = Gas::new();
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        Layout::empty(),
        &ctors,
        include_str!("wasm/runtime_app_ctor.wast").into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app (and invoking a non-`ctor`)
    let name = "My App";
    let ctor = "non-ctor";
    let calldata = vec![];

    let bytes = testing::build_app(version, &template_addr, name, ctor, &calldata);
    let maybe_gas = Gas::new();

    let receipt = runtime.spawn_app(&bytes, &creator, maybe_gas);
    assert!(matches!(
        receipt.error.unwrap(),
        RuntimeError::FuncNotAllowed { .. }
    ));
}

#[ignore = "temporarily disabling this test"]
#[test]
fn default_runtime_spawn_app_with_ctor_reaches_oog() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();
    let maybe_gas = Gas::new();
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        Layout::empty(),
        &ctors,
        include_str!("wasm/runtime_app_ctor.wast").into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app (and invoking its `ctor`)
    let name = "My App";
    let ctor = "ctor";
    let calldata = vec![];

    let bytes = testing::build_app(version, &template_addr, name, ctor, &calldata);
    let maybe_gas = Gas::with(0);

    let expected = SpawnAppReceipt::new_oog(Vec::new());
    let actual = runtime.spawn_app(&bytes, &creator, maybe_gas);
    assert_eq!(expected, actual);
}

#[ignore = "temporarily disabling this test"]
#[test]
fn default_runtime_exec_app_with_ctor_fails() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let maybe_gas = Gas::new();
    let layout: Layout = vec![Address::len() as u32].into();
    let ctors = vec!["initialize".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        layout.clone(),
        &ctors,
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app
    let name = "My App";
    let ctor = "initialize";
    let calldata = vec![];
    let creator = Address::of("creator").into();
    let bytes = testing::build_app(version, &template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn_app(&bytes, &creator, maybe_gas);
    assert!(receipt.success);

    let app_addr = receipt.get_app_addr();
    let init_state = receipt.get_init_state();

    // 3) execute a transaction
    let calldata = Vec::new();
    let bytes = testing::build_app_tx(version, &app_addr, ctor, &calldata);
    let tx = runtime.validate_tx(&bytes).unwrap();

    let receipt = runtime.exec_tx(&tx, &init_state, maybe_gas);

    assert!(matches!(
        receipt.error.unwrap(),
        RuntimeError::FuncNotAllowed { .. }
    ));
}

#[test]
fn default_runtime_calldata_returndata() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let maybe_gas = Gas::new();
    let layout: Layout = vec![Address::len() as u32].into();
    let ctors = vec!["initialize".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        layout.clone(),
        &ctors,
        (&include_bytes!("wasm/runtime_calldata.wasm")[..]).into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app
    let name = "My App";
    let ctor = "initialize";
    let calldata = vec![];
    let creator = Address::of("creator").into();
    let bytes = testing::build_app(version, &template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn_app(&bytes, &creator, maybe_gas);
    assert!(receipt.success);

    let app_addr = receipt.get_app_addr();
    let init_state = receipt.get_init_state();

    // 3) execute a transaction
    // let func = "store_addr";
    // let msg: sdk::Address = sdk::Address::repeat(0x10);

    // let mut calldata = svm_sdk::Vec::with_capacity(100_000);
    // msg.encode(&mut calldata);

    // let bytes = testing::build_app_tx(version, &app_addr, func, &calldata);
    // let tx = runtime.validate_tx(&bytes).unwrap();

    // let receipt = runtime.exec_tx(&tx, &init_state, maybe_gas);
    // assert!(receipt.success);

    // let state = receipt.get_new_state();

    // 4) execute a transaction with `returndata`
    // let func = "load_addr";
    // let calldata = Vec::new();

    // let bytes = testing::build_app_tx(version, &app_addr, func, &calldata);
    // let tx = runtime.validate_tx(&bytes).unwrap();

    // let receipt = runtime.exec_tx(&tx, &state, maybe_gas);
    // assert!(receipt.success);

    // let bytes = receipt.returndata.unwrap();
    // let mut returndata = ReturnData::new(&bytes);

    // let addr: sdk::Address = returndata.next_1();
    // assert_eq!(addr.as_slice(), &[0x10; 20]);
}
