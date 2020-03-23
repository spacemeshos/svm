use std::path::Path;

use svm_app::types::{HostCtx, WasmValue};
use svm_common::Address;
use svm_runtime::{runtime::Runtime, settings::AppSettings, testing};
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

#[test]
fn runtime_spawn_app_with_ctor() {
    // 1) init
    let version = 0;
    let kv = testing::memory_kv_store_init();
    let host = std::ptr::null_mut();
    let imports = Vec::new();
    let mut runtime = testing::create_memory_runtime(host, &kv, imports);
    let page_count = 10;
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();

    // 2) deploying the template
    let bytes = testing::build_template(
        version,
        "My Template",
        page_count,
        include_str!("wasm/runtime_app_ctor.wast"),
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), false);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 3) spawn app (and invoking its `ctor`)
    let buf_size: u32 = 10;
    let ctor_idx = 0;
    let ctor_buf = vec![0xAA, 0xBB, 0xBB, 0xCC, 0xCC, 0xCC, 0xDD, 0xDD, 0xDD, 0xDD];
    let ctor_args = vec![WasmValue::I32(buf_size)];

    let bytes = testing::build_app(version, &template_addr, ctor_idx, &ctor_buf, &ctor_args);

    let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), false);

    let settings = AppSettings {
        page_count,
        kv_path: Path::new("mem").to_path_buf(),
    };

    let mut storage =
        runtime.open_app_storage(receipt.get_app_addr(), receipt.get_init_state(), &settings);

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
    let author = Address::of("author").into();
    let creator = Address::of("creator").into();
    let page_count = 10;

    let kv = testing::memory_kv_store_init();
    let host = std::ptr::null_mut();
    let imports = Vec::new();
    let mut runtime = testing::create_memory_runtime(host, &kv, imports);

    // 2) deploying the template
    let bytes = testing::build_template(
        version,
        "My Template",
        page_count,
        include_str!("wasm/runtime_exec_app.wast"),
    );

    let receipt = runtime.deploy_template(&bytes, &author, HostCtx::new(), false);
    assert!(receipt.success);

    let template_addr = receipt.addr.unwrap();

    // 3) spawn app
    let ctor_idx = 0;
    let ctor_buf = vec![];
    let ctor_args = vec![];

    let bytes = testing::build_app(version, &template_addr, ctor_idx, &ctor_buf, &ctor_args);
    let receipt = runtime.spawn_app(&bytes, &creator, HostCtx::new(), false);

    let app_addr = receipt.get_app_addr();
    let init_state = receipt.get_init_state();

    // // 4) executing the app-transaction
    let buf_id = 0;
    let buf_offset = 0;
    let reg_bits = 128;
    let reg_idx = 3;
    let reg_size = reg_bits / 8;
    let page_idx = 1;
    let page_offset = 20;

    let func_idx = 1;
    let func_buf = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0];
    let count = func_buf.len() as u32;

    assert!(count <= reg_size);

    let func_args = vec![
        WasmValue::I32(buf_id),
        WasmValue::I32(buf_offset),
        WasmValue::I32(reg_bits as u32),
        WasmValue::I32(reg_idx as u32),
        WasmValue::I32(count),
        WasmValue::I32(page_idx),
        WasmValue::I32(page_offset),
    ];
    let bytes = testing::build_app_tx(version, &app_addr, func_idx, &func_buf, &func_args);

    let dry_run = false;
    let receipt = runtime.exec_app(&bytes, &init_state, HostCtx::new(), dry_run);

    assert_eq!(true, receipt.success);
    assert_eq!(None, receipt.error);

    // now we'll read directly from the app's storage
    // and assert that the data has been persisted as expected.

    let settings = AppSettings {
        page_count,
        kv_path: Path::new("mem").to_path_buf(),
    };
    let mut storage = runtime.open_app_storage(app_addr, receipt.get_new_state(), &settings);

    let layout = PageSliceLayout::new(
        PageIndex(page_idx as u16),
        PageOffset(page_offset as u32),
        count,
    );
    let slice = storage.read_page_slice(&layout);

    assert_eq!(func_buf, slice);
}
