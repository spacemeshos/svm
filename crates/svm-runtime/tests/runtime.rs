use svm_app::types::{HostCtx, WasmValue};
use svm_common::Address;
use svm_runtime::{settings::AppSettings, testing, traits::Runtime};
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

#[test]
fn runtime_spawn_app_with_ctor() {
    // 1) init
    let version = 0;
    let kv = testing::memory_kv_store_init();
    let host = std::ptr::null_mut();
    let imports = Vec::new();
    let mut runtime = testing::create_memory_runtime(host, &kv, imports);
    let pages_count = 10;
    let author = Address::of("author");
    let creator = Address::of("creator");

    // 2) deploying the template
    let bytes = testing::build_template(
        version,
        "Template #1",
        pages_count,
        include_str!("wasm/runtime_app_ctor.wast"),
    );

    let template_addr = runtime
        .deploy_template(&author, HostCtx::new(), &bytes)
        .unwrap();

    // 3) spawn app (and invoking its `ctor`)
    let buf_size: u32 = 10;
    let ctor_buf = vec![
        vec![0xAA],
        vec![0xBB, 0xBB],
        vec![0xCC, 0xCC, 0xCC],
        vec![0xDD, 0xDD, 0xDD, 0xDD],
    ];

    let ctor_args = vec![WasmValue::I32(buf_size as i32)];
    let bytes = testing::build_app(version, &template_addr, &ctor_buf, &ctor_args);

    let (app_addr, init_state) = runtime.spawn_app(&creator, HostCtx::new(), &bytes).unwrap();

    let settings = AppSettings { pages_count };
    let mut storage = runtime.open_app_storage(&app_addr, &init_state, &settings);

    let layout = PageSliceLayout::new(PageIndex(0), PageOffset(0), buf_size);
    let slice = storage.read_page_slice(&layout);

    assert_eq!(
        vec![0xAA, 0xBB, 0xBB, 0xCC, 0xCC, 0xCC, 0xDD, 0xDD, 0xDD, 0xDD],
        slice
    );
}

#[test]
fn runtime_exec_app() {
    // 1) init
    let version = 0;
    let kv = testing::memory_kv_store_init();
    let host = std::ptr::null_mut();
    let imports = Vec::new();
    let mut runtime = testing::create_memory_runtime(host, &kv, imports);
    let pages_count = 10;
    let author = Address::of("author");
    let creator = Address::of("creator");
    let sender = Address::of("sender");

    // 2) deploying the template
    let bytes = testing::build_template(
        version,
        "Template #1",
        pages_count,
        include_str!("wasm/runtime_exec_app.wast"),
    );

    let template_addr = runtime
        .deploy_template(&author, HostCtx::new(), &bytes)
        .unwrap();

    // 3) spawn app
    let ctor_buf = vec![];
    let ctor_args = vec![];
    let bytes = testing::build_app(version, &template_addr, &ctor_buf, &ctor_args);

    let (app_addr, init_state) = runtime.spawn_app(&creator, HostCtx::new(), &bytes).unwrap();

    // 4) executing the app-transaction.
    let func_name = "run";
    let func_buf = vec![vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]];
    let func_args = vec![
        WasmValue::I32(0),  // buf_offset
        WasmValue::I32(64), // reg_bits
        WasmValue::I32(0),  // reg_idx
        WasmValue::I32(8),  // len
    ];
    let bytes = testing::build_app_tx(version, &app_addr, func_name, &func_buf, &func_args);

    let tx = runtime.parse_exec_app(&sender, &bytes).unwrap();
    let res = runtime.exec_app(tx, init_state.clone(), HostCtx::new());

    let receipt = res.unwrap();

    assert_eq!(true, receipt.success);
    assert_eq!(None, receipt.error);

    let new_state = receipt.new_state.as_ref().unwrap();

    // now we'll read directly from the app's storage and assert that the
    // data has been persisted as expected.

    let settings = AppSettings { pages_count };
    let mut storage = runtime.open_app_storage(&app_addr, new_state, &settings);

    let layout = PageSliceLayout::new(PageIndex(0), PageOffset(0), 8);
    let slice = storage.read_page_slice(&layout);

    assert_eq!(vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80], slice);
}
