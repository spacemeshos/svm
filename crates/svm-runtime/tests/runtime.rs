use std::collections::HashMap;

use svm_app::types::WasmArgValue as Value;
use svm_common::{Address, State};
use svm_runtime::{settings::AppSettings, testing, traits::Runtime};
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

#[test]
fn runtime_valid_app_transaction() {
    // 1) init
    let version = 0;
    let kv = testing::memory_kv_store_init();
    let host = std::ptr::null_mut();
    let imports = Vec::new();
    let mut runtime = testing::create_memory_runtime(host, &kv, imports);
    let pages_count = 10;
    let author = 0x10_20_30_40;

    // 2) deploying the template
    let bytes = testing::build_template(
        version,
        "Template #1",
        author,
        pages_count,
        include_str!("wasm/runtime-1.wast"),
    );

    let template_addr = runtime
        .deploy_template(&Address::from(author), &bytes)
        .unwrap();
    let creator = 0x10_20_30_40;

    // 3) spawn app
    let bytes = testing::build_app(version, &template_addr, creator);
    let app_addr = runtime.spawn_app(&Address::from(creator), &bytes).unwrap();

    // 4) executing the app-transaction.
    let sender_addr = 0x50_60_70_80;
    let func_name = "reg_set_and_persist";
    let func_args = vec![
        Value::I64(0x10_20_30_40_50_60_70_80),
        Value::I32(64),
        Value::I32(0),
        Value::I32(0),
        Value::I32(0),
    ];
    let bytes = testing::build_app_tx(version, &app_addr, sender_addr, func_name, &func_args);

    let tx = runtime.parse_exec_app(&bytes).unwrap();
    let res = runtime.exec_app(tx, State::empty(), HashMap::new());

    let receipt = res.unwrap();
    assert_eq!(true, receipt.success);
    assert_eq!(None, receipt.error);

    let new_state = receipt.new_state.unwrap();
    assert_ne!(State::from(0), new_state);

    // now we'll read directly from the app's storage and assert that the
    // data has been persisted as expected.

    let settings = AppSettings { pages_count };
    let mut storage = runtime.open_app_storage(&app_addr, &new_state, &settings);

    let layout = PageSliceLayout::new(PageIndex(0), PageOffset(0), 8);
    let slice = storage.read_page_slice(&layout);

    assert_eq!(vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80], slice);
}
