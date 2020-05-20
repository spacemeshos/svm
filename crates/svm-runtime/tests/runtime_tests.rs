use svm_app::{
    error::ParseError,
    raw::Field,
    types::{HostCtx, WasmValue},
};
use svm_common::Address;
use svm_gas::error::ProgramError;
use svm_layout::{DataLayout, VarId};
use svm_runtime::{
    error::ValidateError,
    gas::MaybeGas,
    receipt::{ExecReceipt, SpawnAppReceipt, TemplateReceipt},
    testing, Runtime,
};

macro_rules! default_runtime {
    () => {{
        use svm_runtime::testing;

        let raw_kv = testing::memory_kv_store_init();

        let host = std::ptr::null_mut();
        let imports = Vec::new();

        testing::create_memory_runtime(host, &raw_kv, imports)
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
    let is_wast = true;

    // invalid wasm (has floats)
    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        include_str!("wasm/wasm_with_floats.wast"),
        is_wast,
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

    let parse_err = ParseError::NotEnoughBytes(Field::AppTemplate);
    let expected = Err(ValidateError::Parse(parse_err));

    let actual = runtime.validate_app(&bytes);
    assert_eq!(expected, actual);
}

#[test]
fn default_runtime_validate_tx_invalid_raw_format() {
    let runtime = default_runtime!();

    let bytes = vec![0xFF, 0xFF];

    let parse_err = ParseError::NotEnoughBytes(Field::App);
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
    let is_wast = true;

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        include_str!("wasm/runtime_app_ctor.wast"),
        is_wast,
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
    let is_wast = true;

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        include_str!("wasm/runtime_app_ctor.wast"),
        is_wast,
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
    let is_wast = true;
    let maybe_gas = MaybeGas::new();

    let bytes = testing::build_template(
        version,
        "My Template",
        DataLayout::empty(),
        include_str!("wasm/runtime_app_ctor.wast"),
        is_wast,
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app (and invoking its `ctor`)
    let ctor_idx = 0;
    let ctor_buf = vec![];
    let ctor_args = vec![];

    let bytes = testing::build_app(version, &template_addr, ctor_idx, &ctor_buf, &ctor_args);

    let maybe_gas = MaybeGas::with(0);

    let expected = SpawnAppReceipt::new_oog();
    let actual = runtime.spawn_app(&bytes, &creator, HostCtx::new(), maybe_gas);
    assert_eq!(expected, actual);
}

#[test]
fn runtime_spawn_app_with_ctor_with_enough_gas() {
    let mut runtime = default_runtime!();

    // 1) deploying the template
    let version = 0;
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();
    let is_wast = true;
    let maybe_gas = MaybeGas::new();

    // data layout consists on one variable of 8 bytes (offsets: `[0..8)`)
    let layout: DataLayout = vec![8].into();

    let bytes = testing::build_template(
        version,
        "My Template",
        layout.clone(),
        include_str!("wasm/runtime_app_ctor.wast"),
        is_wast,
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());

    let template_addr = receipt.addr.unwrap();

    // 2) spawn app (and invoking its `ctor`)
    let ctor_func_idx = 0;
    let ctor_buf = vec![];
    let ctor_args = vec![WasmValue::I64(10_20_30_40_50_60_70_80)];
    let bytes = testing::build_app(
        version,
        &template_addr,
        ctor_func_idx,
        &ctor_buf,
        &ctor_args,
    );
    let gas_limit = MaybeGas::with(1_000_000);

    let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), gas_limit);
    assert!(receipt.success);
    assert!(receipt.gas_used.is_some());

    let addr = receipt.get_app_addr();
    let state = receipt.get_init_state();
    let storage = runtime.open_app_storage(&addr, &state, &layout);

    let var = storage.read_var(VarId(0));
    assert_eq!(var, 10_20_30_40_50_60_70_80u64.to_be_bytes());
}

// #[test]
// fn runtime_exec_app() {
//     let mut runtime = default_runtime!();

//     // 1) deploying the template
//     let version = 0;
//     let author = Address::of("author").into();
//     let creator = Address::of("creator").into();
//     let page_count = 10;
//     let is_wast = true;
//     let maybe_gas = MaybeGas::new();

//     let bytes = testing::build_template(
//         version,
//         "My Template",
//         page_count,
//         DataLayout::empty(),
//         include_str!("wasm/runtime_exec_app.wast"),
//         is_wast,
//     );

//     let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
//     assert!(receipt.success);

//     let template_addr = receipt.addr.unwrap();

//     // 2) spawn app
//     let ctor_idx = 0;
//     let ctor_buf = vec![];
//     let ctor_args = vec![];

//     let bytes = testing::build_app(version, &template_addr, ctor_idx, &ctor_buf, &ctor_args);
//     let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), maybe_gas);

//     let app_addr = receipt.get_app_addr();
//     let init_state = receipt.get_init_state();

//     // 3) executing the app-transaction
//     let buf_id = 0;
//     let buf_offset = 0;
//     let reg_bits = 128;
//     let reg_idx = 3;
//     let reg_size = reg_bits / 8;
//     let page_idx = 1;
//     let page_offset = 20;

//     let func_idx = 1;
//     let func_buf = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0];
//     let count = func_buf.len() as u32;

//     assert!(count <= reg_size);

//     let func_args = vec![
//         WasmValue::I32(buf_id),
//         WasmValue::I32(buf_offset),
//         WasmValue::I32(reg_bits as u32),
//         WasmValue::I32(reg_idx as u32),
//         WasmValue::I32(count),
//         WasmValue::I32(page_idx),
//         WasmValue::I32(page_offset),
//     ];
//     let bytes = testing::build_app_tx(version, &app_addr, func_idx, &func_buf, &func_args);

//     let receipt = runtime.exec_app(&bytes, &init_state, HostCtx::new(), maybe_gas);

//     assert!(receipt.success);
//     assert!(receipt.error.is_none());

//     // now we'll read directly from the app's storage
//     // and assert that the data has been persisted as expected.

//     let settings = AppSettings {
//         page_count,
//         layout: DataLayout::empty(),
//         kv_path: Path::new("mem").to_path_buf(),
//     };

//     let mut storage = runtime.open_app_storage(app_addr, receipt.get_new_state(), &settings);

//     let layout = PageSliceLayout::new(
//         PageIndex(page_idx as u16),
//         PageOffset(page_offset as u32),
//         count,
//     );
//     let slice = storage.read_page_slice(&layout);

//     assert_eq!(func_buf, slice);
// }

// #[test]
// fn runtime_exec_app_reaches_oog() {
//     let mut runtime = default_runtime!();

//     // 1) deploying the template
//     let version = 0;
//     let author = Address::of("author").into();
//     let creator = Address::of("creator").into();
//     let page_count = 10;
//     let is_wast = true;
//     let maybe_gas = MaybeGas::new();

//     let bytes = testing::build_template(
//         version,
//         "My Template",
//         page_count,
//         DataLayout::empty(),
//         include_str!("wasm/runtime_exec_app.wast"),
//         is_wast,
//     );

//     let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), maybe_gas);
//     assert!(receipt.success);

//     let template_addr = receipt.addr.unwrap();

//     // 2) spawn app
//     let ctor_idx = 0;
//     let ctor_buf = vec![];
//     let ctor_args = vec![];

//     let bytes = testing::build_app(version, &template_addr, ctor_idx, &ctor_buf, &ctor_args);
//     let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), maybe_gas);

//     let app_addr = receipt.get_app_addr();
//     let init_state = receipt.get_init_state();

//     // 3) executing the app-transaction
//     let buf_id = 0;
//     let buf_offset = 0;
//     let reg_bits = 128;
//     let reg_idx = 3;
//     let reg_size = reg_bits / 8;
//     let page_idx = 1;
//     let page_offset = 20;

//     let func_idx = 1;
//     let func_buf = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0];
//     let count = func_buf.len() as u32;

//     assert!(count <= reg_size);

//     let func_args = vec![
//         WasmValue::I32(buf_id),
//         WasmValue::I32(buf_offset),
//         WasmValue::I32(reg_bits as u32),
//         WasmValue::I32(reg_idx as u32),
//         WasmValue::I32(count),
//         WasmValue::I32(page_idx),
//         WasmValue::I32(page_offset),
//     ];
//     let bytes = testing::build_app_tx(version, &app_addr, func_idx, &func_buf, &func_args);

//     let maybe_gas = MaybeGas::with(0);

//     let expected = ExecReceipt::new_oog();
//     let actual = runtime.exec_app(&bytes, &init_state, HostCtx::new(), maybe_gas);

//     assert_eq!(expected, actual)
// }
