use svm_abi_decoder::{Cursor, Decoder};
use svm_abi_encoder::Encoder;

use svm_sdk::Address as AbiAddr;

use svm_codec::{Field, ParseError};

use svm_gas::error::ProgramError;
use svm_layout::{DataLayout, VarId};
use svm_runtime::{error::ValidateError, testing, Runtime};

use svm_types::receipt::{ExecReceipt, Log, SpawnAppReceipt, TemplateReceipt};
use svm_types::{gas::MaybeGas, Address};

macro_rules! default_runtime {
    () => {{
        use svm_runtime::testing;

        let state_kv = testing::memory_state_kv_init();
        let imports = Vec::new();

        let imports = Box::leak(Box::new(imports));

        testing::create_memory_runtime(&state_kv, imports)
    }};
}

#[test]
#[ignore]
fn default_runtime_validate_template_invalid_raw_format() {
    let runtime = default_runtime!();
    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::Name);
    let expected = ValidateError::Parse(parse_err);

    let actual = runtime.validate_template(&bytes[..]).unwrap_err();
    assert_eq!(expected, actual);
}

#[test]
fn default_runtime_validate_template_invalid_wasm() {
    let runtime = default_runtime!();
    let version = 0;
    let ctors = Vec::new();

    // invalid wasm (has floats)
    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
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

#[test]
fn default_runtime_validate_tx_invalid_raw_format() {
    let runtime = default_runtime!();

    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::AppAddr);
    let expected = Err(ValidateError::Parse(parse_err));

    let actual = runtime.validate_tx(&bytes);
    assert_eq!(expected, actual);
}

#[test]
fn default_runtime_deploy_template_reaches_oog() {
    let mut runtime = default_runtime!();

    let version = 0;
    let author = Address::of("author").into();
    let maybe_gas = MaybeGas::with(0);
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        &ctors,
        include_str!("wasm/runtime_app_ctor.wast").into(),
    );

    let expected = TemplateReceipt::new_oog();
    let actual = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert_eq!(expected, actual);
}

#[test]
fn default_runtime_deploy_template_has_enough_gas() {
    let mut runtime = default_runtime!();

    let version = 0;
    let author = Address::of("author").into();
    let gas_limit = MaybeGas::with(1_0000_000);
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        &ctors,
        include_str!("wasm/runtime_app_ctor.wast").into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, gas_limit);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());
}

#[test]
fn default_runtime_spawn_app_with_ctor_reaches_oog() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();
    let maybe_gas = MaybeGas::new();
    let ctors = vec!["ctor".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
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
    let maybe_gas = MaybeGas::with(0);

    let log = Log {
        msg: b"not enough gas (installation_gas = 33000) for installation".to_vec(),
        code: 1,
    };

    let expected = SpawnAppReceipt::new_oog(vec![log]);
    let actual = runtime.spawn_app(&bytes, &creator, maybe_gas);
    assert_eq!(expected, actual);
}

#[test]
#[ignore = "temporarily skipping this test until wasmer cranelift will support middlewares"]
fn default_runtime_spawn_app_with_ctor_with_enough_gas() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();
    let maybe_gas = MaybeGas::new();
    let ctors = vec!["ctor".to_string()];

    // raw layout consists on one variable of 8 bytes (offsets: `[0..8)`)
    let layout: DataLayout = vec![8].into();

    let bytes = testing::build_template(
        version,
        "My Template",
        layout.clone(),
        &ctors,
        include_str!("wasm/runtime_app_ctor.wast").into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app (and invoking its `ctor`)
    let name = "My App";
    let ctor = "ctor";
    let calldata = vec![];
    let bytes = testing::build_app(version, &template_addr, name, ctor, &calldata);
    let gas_limit = MaybeGas::with(1_000_000);

    let receipt = runtime.spawn_app(&bytes, &creator, gas_limit);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());

    let addr = receipt.get_app_addr();
    let state = receipt.get_init_state();
    let storage = runtime.open_app_storage(&addr, &state, &layout);

    let var = storage.read_var(VarId(0));
    assert_eq!(var, 10_20_30_40_50_60_70_80u64.to_le_bytes());
}

#[test]
fn default_runtime_calldata_returndata() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let maybe_gas = MaybeGas::new();
    let layout: DataLayout = vec![20].into();
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
    let func = "store_addr";
    let msg: AbiAddr = [0x10; 20].into();

    let mut calldata = Vec::new();
    msg.encode(&mut calldata);

    let bytes = testing::build_app_tx(version, &app_addr, func, &calldata);

    let receipt = runtime.exec_app(&bytes, &init_state, maybe_gas);
    assert!(receipt.success);

    let state = receipt.get_new_state();

    // 4) execute a transaction with `returndata`
    let func = "return_addr";
    let calldata = vec![];

    let bytes = testing::build_app_tx(version, &app_addr, func, &calldata);

    let receipt = runtime.exec_app(&bytes, &state, maybe_gas);
    assert!(receipt.success);

    let raw = receipt.returndata.unwrap();
    let mut cursor = Cursor::new(&raw);
    let decoder = Decoder::new();

    let decoded = decoder.decode_value(&mut cursor).unwrap();
    let addr: AbiAddr = decoded.into();
    assert_eq!(addr.as_slice(), &[0x10; 20]);
}

#[test]
#[ignore = "temporarily skipping this test until wasmer cranelift will support middlewares"]
fn default_runtime_exec_app_reaches_oog() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();
    let maybe_gas = MaybeGas::new();
    let layout: DataLayout = vec![4].into();
    let ctors = vec!["ctors".to_string()];

    let bytes = testing::build_template(
        version,
        "My Template",
        layout,
        &ctors,
        include_str!("wasm/runtime_exec_app.wast").into(),
    );

    let receipt = runtime.deploy_template(&bytes, &author, maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app
    let name = "My App";
    let ctor = "ctor";
    let calldata = vec![];

    let bytes = testing::build_app(version, &template_addr, name, ctor, &calldata);
    let receipt = runtime.spawn_app(&bytes, &creator, maybe_gas);

    let app_addr = receipt.get_app_addr();
    let init_state = receipt.get_init_state();

    // 3) executing an app-transaction (reaching out-of-gas)
    let func = "add";
    let calldata = vec![];
    let bytes = testing::build_app_tx(version, &app_addr, func, &calldata);
    let maybe_gas = MaybeGas::with(0);
    let logs = Vec::new();

    let expected = ExecReceipt::new_oog(logs);
    let actual = runtime.exec_app(&bytes, &init_state, maybe_gas);

    assert_eq!(expected, actual)
}
