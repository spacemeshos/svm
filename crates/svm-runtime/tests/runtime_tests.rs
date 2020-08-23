use svm_abi_encoder::Encoder;

use svm_codec::api::raw::Field;
use svm_codec::error::ParseError;

use svm_gas::error::ProgramError;
use svm_layout::{DataLayout, VarId};
use svm_runtime::{error::ValidateError, testing, testing::WasmFile, Runtime};

use svm_types::receipt::{ExecReceipt, Log, SpawnAppReceipt, TemplateReceipt};
use svm_types::{gas::MaybeGas, Address, HostCtx};

use svm_sdk::value::AddressOwned;

macro_rules! default_runtime {
    () => {{
        use svm_runtime::testing;

        let state_kv = testing::memory_state_kv_init();

        let host = std::ptr::null_mut();
        let imports = Vec::new();

        testing::create_memory_runtime(host, &state_kv, imports)
    }};
}

#[test]
fn default_runtime_validate_template_invalid_raw_format() {
    let runtime = default_runtime!();
    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::NameLength);
    let expected = Err(ValidateError::Parse(parse_err));

    let actual = runtime.validate_template(&bytes[..]);
    assert_eq!(expected, actual);
}

#[test]
fn default_runtime_validate_template_invalid_wasm() {
    let runtime = default_runtime!();

    let version = 0;

    // invalid wasm (has floats)
    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        WasmFile::Text(include_str!("wasm/wasm_with_floats.wast")),
    );

    let prog_err = ProgramError::FloatsNotAllowed;
    let expected = Err(ValidateError::Program(prog_err));

    let actual = runtime.validate_template(&bytes[..]);
    assert_eq!(expected, actual);
}

#[test]
fn default_runtime_validate_app_invalid_raw_format() {
    let runtime = default_runtime!();
    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::TemplateAddr);
    let expected = Err(ValidateError::Parse(parse_err));

    let actual = runtime.validate_app(&bytes);
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

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        WasmFile::Text(include_str!("wasm/runtime_app_ctor.wast")),
    );

    let expected = TemplateReceipt::new_oog();
    let actual = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
    assert_eq!(expected, actual);
}

#[test]
fn default_runtime_deploy_template_has_enough_gas() {
    let mut runtime = default_runtime!();

    let version = 0;
    let author = Address::of("author").into();
    let gas_limit = MaybeGas::with(1_0000_000);

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        WasmFile::Text(include_str!("wasm/runtime_app_ctor.wast")),
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), gas_limit);
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

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        WasmFile::Text(include_str!("wasm/runtime_app_ctor.wast")),
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app (and invoking its `ctor`)
    let name = "My App";
    let ctor_idx = 0;
    let calldata = vec![];

    let bytes = testing::build_app(version, &template_addr, name, ctor_idx, &calldata);
    let maybe_gas = MaybeGas::with(0);

    let log = Log {
        msg: b"not enough gas (installation_gas = 29000) for installation".to_vec(),
        code: 1,
    };

    let expected = SpawnAppReceipt::new_oog(vec![log]);
    let actual = runtime.spawn_app(&bytes, &creator, HostCtx::new(), maybe_gas);
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

    // data layout consists on one variable of 8 bytes (offsets: `[0..8)`)
    let layout: DataLayout = vec![8].into();

    let bytes = testing::build_template(
        version,
        "My Template",
        layout.clone(),
        WasmFile::Text(include_str!("wasm/runtime_app_ctor.wast")),
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app (and invoking its `ctor`)
    let name = "My App";
    let ctor_func_idx = 0;
    let calldata = vec![];
    let bytes = testing::build_app(version, &template_addr, name, ctor_func_idx, &calldata);
    let gas_limit = MaybeGas::with(1_000_000);

    let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), gas_limit);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());

    let addr = receipt.get_app_addr();
    let state = receipt.get_init_state();
    let storage = runtime.open_app_storage(&addr, &state, &layout);

    let var = storage.read_var(VarId(0));
    assert_eq!(var, 10_20_30_40_50_60_70_80u64.to_le_bytes());
}

#[test]
fn default_runtime_calldata() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let maybe_gas = MaybeGas::new();
    let layout: DataLayout = vec![20].into();

    let bytes = testing::build_template(
        version,
        "My Template",
        layout.clone(),
        WasmFile::Binary(include_bytes!("wasm/runtime_calldata.wasm")),
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app
    let name = "My App";
    let ctor_idx = 1; // initialize
    let calldata = vec![];
    let creator = Address::of("creator").into();
    let bytes = testing::build_app(version, &template_addr, name, ctor_idx, &calldata);
    let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), maybe_gas);
    assert!(receipt.success);

    let app_addr = receipt.get_app_addr();
    let init_state = receipt.get_init_state();

    // 3) executing an app-transaction
    let func_idx = 3;
    let msg = AddressOwned([0x10; 20]);

    let mut calldata = Vec::new();
    msg.encode(&mut calldata);

    let bytes = testing::build_app_tx(version, &app_addr, func_idx, &calldata);

    let receipt = runtime.exec_app(&bytes, &init_state, HostCtx::new(), maybe_gas);
    assert!(receipt.success);

    // now we'll read directly from the app's storage
    // and assert that the data has been persisted as expected.

    let state = receipt.get_new_state();
    let storage = runtime.open_app_storage(&app_addr, &state, &layout);

    let var = storage.read_var(VarId(0));
    assert_eq!(var, [0x10; 20]);
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

    let bytes = testing::build_template(
        version,
        "My Template",
        layout,
        WasmFile::Text(include_str!("wasm/runtime_exec_app.wast")),
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app
    let name = "My App";
    let ctor_idx = 0;
    let calldata = vec![];

    let bytes = testing::build_app(version, &template_addr, name, ctor_idx, &calldata);
    let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), maybe_gas);

    let app_addr = receipt.get_app_addr();
    let init_state = receipt.get_init_state();

    // 3) executing an app-transaction (reaching out-of-gas)
    let func_idx = 1;
    let calldata = vec![];
    let bytes = testing::build_app_tx(version, &app_addr, func_idx, &calldata);
    let maybe_gas = MaybeGas::with(0);
    let logs = Vec::new();

    let expected = ExecReceipt::new_oog(logs);
    let actual = runtime.exec_app(&bytes, &init_state, HostCtx::new(), maybe_gas);

    assert_eq!(expected, actual)
}
