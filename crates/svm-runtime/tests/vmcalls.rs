use std::ffi::c_void;

use maplit::hashmap;

use wasmer_runtime::{func, imports, Func};

use svm_app::types::HostCtx;
use svm_common::{Address, State};
use svm_runtime::{
    helpers::{self, DataWrapper},
    testing::{self, instance_buffer, instance_register, instance_storage},
    vmcalls,
};
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

fn default_test_args() -> (
    Address,
    State,
    DataWrapper<*mut c_void>,
    DataWrapper<*const c_void>,
    u16,
) {
    let app_addr = Address::of("my-app");
    let state = State::from(0x_00_00_00_00);
    let host = DataWrapper::new(std::ptr::null_mut());
    let host_ctx = DataWrapper::new(svm_common::into_raw(HostCtx::new()));
    let pages_count = 5;

    (app_addr, state, host, host_ctx, pages_count)
}

#[test]
fn vmcalls_empty_wasm() {
    let wasm = r#"
        (module
          (func (export "run")))"#;

    testing::instantiate(&imports! {}, wasm);
}

#[test]
fn vmcalls_mem_to_reg_copy() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "mem_to_reg_copy" => func!(vmcalls::mem_to_reg_copy),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/mem_to_reg_copy.wast"));

    // initializing memory #0 cells `200..203` with values `10, 20, 30` respectively
    testing::instance_memory_init(&instance, 200, &[10, 20, 30]);

    // asserting register `64:2` content is initialized with zeros
    let reg = instance_register(&instance, 64, 2);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0], reg.view());

    let func: Func<(i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(200, 3, 2).is_ok());

    // asserting register `64:2` content is `10, 20, 30, 0, 0, ... 0`
    let reg = instance_register(&instance, 64, 2);
    assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_reg_to_mem_copy() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "reg_to_mem_copy" => func!(vmcalls::reg_to_mem_copy),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/reg_to_mem_copy.wast"));

    // initializing reg `64:2` with values `10, 20, 30` respectively
    let reg = instance_register(&instance, 64, 2);
    reg.set(&[10, 20, 30]);

    // asserting memory #0, cells `0..3` are zeros before copy
    let cells = testing::instance_memory_view(&instance, 0, 3);
    assert_eq!(vec![0, 0, 0], cells);

    // copying reg `64:2` content into memory cells `0..3`
    let func: Func<(i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(2, 3, 0).is_ok());

    // asserting memory #0, cells `0..3` have the values `10, 20, 30` respectively
    let cells = testing::instance_memory_view(&instance, 0, 3);
    assert_eq!(vec![10, 20, 30], cells);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_reg() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_reg_copy.wast"),
    );

    // we first initialize register `2:64` with some garbage data (0xFF...FF) which should be overriden
    // after calling the exported `run` function
    let reg = instance_register(&instance, 64, 2);
    reg.set(&[0xFF; 8]);

    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(1, 100, 3, 2).is_ok());

    // register `64:2` should contain zeros, since an empty page-slice is treated as a page-slice containing only zeros
    let reg = instance_register(&instance, 64, 2);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_reg() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_reg_copy.wast"),
    );

    let storage = instance_storage(&instance);
    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    // we write `[10, 20, 30]` into storage slice (page `1`, cells: `100..103`)
    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    // we first initialize register `64:2` with some garbage (0xFF...FF) data which should be overriden
    // after calling the exported `run` function
    let reg = instance_register(&instance, 64, 2);
    reg.set(&[0xFF; 8]);

    // we copy slice (page `1`, cells: `100..103`) into register `2:64`
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(1, 100, 3, 2).is_ok());

    let reg = instance_register(&instance, 64, 2);
    assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_mem() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "storage_read_to_mem" => func!(vmcalls::storage_read_to_mem),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_mem_copy.wast"),
    );

    // we fill memory #0, cells `200..203` with garbage data (0xFF...FF)
    testing::instance_memory_init(&instance, 200, &[0xFF, 0xFF, 0xFF]);

    let cells = testing::instance_memory_view(&instance, 200, 3);
    assert_eq!(vec![0xFF, 0xFF, 0xFF], cells);

    // we copy storage slice (page `1`, cells: `100...103`) into memory `#0` starting cells `200...203`
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(1, 100, 3, 200).is_ok());

    let cells = testing::instance_memory_view(&instance, 200, 3);
    assert_eq!(vec![0, 0, 0], cells);
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_mem() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "storage_read_to_mem" => func!(vmcalls::storage_read_to_mem),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_mem_copy.wast"),
    );

    let storage = instance_storage(&instance);
    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    // we write `[10, 20, 30]` into storage slice (page `1`, cells `100..103`)
    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    // we copy slice (page `1`, cells: `100..103`) into memory #0, starting from address `200`
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(1, 100, 3, 200).is_ok());

    let cells = testing::instance_memory_view(&instance, 200, 3);
    assert_eq!(vec![10, 20, 30], cells);
}

#[test]
fn vmcalls_storage_write_from_mem() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "storage_write_from_mem" => func!(vmcalls::storage_write_from_mem),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_write_from_mem.wast"),
    );

    // initializing memory `#0` cells `200...203` with `10, 20, 30` respectively
    testing::instance_memory_init(&instance, 200, &[10, 20, 30]);

    let storage = instance_storage(&instance);
    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    assert_eq!(vec![0, 0, 0], storage.read_page_slice(&layout));

    // we copy memory cells `200..`203` into storage (`page 1`, cells: `100..103`)
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(200, 3, 1, 100).is_ok());

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_storage_write_from_reg() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "storage_write_from_reg" => func!(vmcalls::storage_write_from_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_write_from_reg.wast"),
    );

    let storage = instance_storage(&instance);

    // we first initialize register `64:5` with `[10, 20, 30, 0, 0, 0, 0, 0]`
    let reg = instance_register(&instance, 64, 5);
    reg.set(&[10, 20, 30]);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(200), 3);
    assert_eq!(vec![0, 0, 0], storage.read_page_slice(&layout));

    // we copy register `64:5` first into storage (`page 1`, cells: `200..203`)
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(5, 3, 1, 200).is_ok());

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_reg_replace_byte_read_write_be_i64() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
            "storage_write_from_reg" => func!(vmcalls::storage_write_from_reg),
            "reg_replace_byte" => func!(vmcalls::reg_replace_byte),
            "reg_read_be_i64" => func!(vmcalls::reg_read_be_i64),
            "reg_write_be_i64" => func!(vmcalls::reg_write_be_i64),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/reg_replace_read_write_be_i64.wast"),
    );

    // we first initialize register `64:5` with `[254, 255, 0, 0, 0, 0, 0, 0]`
    let reg = instance_register(&instance, 64, 5);
    reg.set(&[0, 0, 0, 0, 0, 0, 255, 254]);

    let inc: Func<i32> = instance.func("inc").unwrap();
    assert!(inc.call(5).is_ok());

    let reg = instance_register(&instance, 64, 5);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 255, 255], reg.view());

    let func: Func<i32> = instance.func("inc").unwrap();
    assert!(func.call(5).is_ok());

    let reg = instance_register(&instance, 64, 5);
    assert_eq!(vec![0, 0, 0, 0, 0, 1, 0, 0], reg.view());

    // now we'll change 2 bytes of register `64:5`
    let func: Func<(i32, i32, i32)> = instance.func("replace").unwrap();
    assert!(func.call(5, 10, 6).is_ok());
    assert!(func.call(5, 20, 7).is_ok());

    let reg = instance_register(&instance, 64, 5);
    assert_eq!(vec![0, 0, 0, 0, 0, 1, 10, 20], reg.view());
}

#[test]
fn vmcalls_host_ctx_read_into_reg() {
    let (app_addr, state, host, _host_ctx, pages_count) = default_test_args();

    let host_ctx = HostCtx::from(hashmap! {
        2 => vec![10, 20],
        3 => vec![30, 40, 50]
    });

    let host_ctx = DataWrapper::new(svm_common::into_raw(host_ctx));

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "host_ctx_read_into_reg" => func!(vmcalls::host_ctx_read_into_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/host_ctx_read_into_reg.wast"),
    );

    let func: Func<(i32, i32, i32)> = instance.func("run").unwrap();

    // copying field `2` (content=`[10, 20]`) into register `64:3`
    assert!(func.call(2, 64, 3).is_ok());

    // copying field `3` (content=`[30, 40, 50]`) into register `32:5`
    assert!(func.call(3, 32, 5).is_ok());

    let reg = instance_register(&instance, 64, 3);
    assert_eq!(vec![10, 20, 0, 0, 0, 0, 0, 0], reg.view());

    let reg = instance_register(&instance, 32, 5);
    assert_eq!(vec![30, 40, 50, 0], reg.view());
}

#[test]
fn vmcalls_buffer_copy_to_storage() {
    let (app_addr, state, host, host_ctx, pages_count) = default_test_args();

    // app needs at least 2 pages for this test
    assert!(pages_count >= 2);

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, pages_count),

        "svm" => {
            "buffer_create" => func!(vmcalls::buffer_create),
            "buffer_kill" => func!(vmcalls::buffer_kill),
            "buffer_copy_to_storage" => func!(vmcalls::buffer_copy_to_storage),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/buffer.wast"));

    // create buffer with `index=2`
    let func: Func<i32> = instance.func("create").unwrap();
    assert!(func.call(2).is_ok());

    // create buffer with `index=5`
    let func: Func<i32> = instance.func("create").unwrap();
    assert!(func.call(5).is_ok());

    let buf2 = instance_buffer(&instance, 2).unwrap();
    let buf5 = instance_buffer(&instance, 5).unwrap();

    buf2.write(&[10, 20, 30]); // write to buf #2 to locations: `0, 1, 2`
    buf2.write(&[100, 200]); // write to buf #2 to locations: `3, 4`
    buf5.write(&[40, 50, 60, 70]); // write to buf #5 to locations: `0, 1, 2, 3`

    // copy buf_id=2, buf_offset=0 into page #0 bytes `4, 5, 6` (page_offset=4, len=3)
    let func: Func<(i32, i32, i32, i32, i32)> = instance.func("copy").unwrap();
    assert!(func.call(2, 0, 0, 4, 3).is_ok());

    // copy buf_id=2, buf_offset=3 buf into page #0 bytes `7, 8` (page_offset=7, len=2)
    let func: Func<(i32, i32, i32, i32, i32)> = instance.func("copy").unwrap();
    assert!(func.call(2, 3, 0, 7, 2).is_ok());

    // copy buf_id=5, buf_offset=0 into page #1 bytes `0, 1, 2, 3` (page_offset=0, len=4)
    let func: Func<(i32, i32, i32, i32, i32)> = instance.func("copy").unwrap();
    assert!(func.call(5, 0, 1, 0, 4).is_ok());

    // killing buffers #2 and #5
    assert!(instance_buffer(&instance, 2).is_some());
    assert!(instance_buffer(&instance, 5).is_some());

    let func: Func<i32> = instance.func("kill").unwrap();
    assert!(func.call(2).is_ok());
    assert!(func.call(5).is_ok());

    assert!(instance_buffer(&instance, 2).is_none());
    assert!(instance_buffer(&instance, 5).is_none());

    // asserting persisted storage

    let storage = instance_storage(&instance);
    assert_eq!(
        vec![10, 20, 30],
        helpers::storage_read_page_slice(storage, 0, 4, 3)
    );

    assert_eq!(
        vec![100, 200],
        helpers::storage_read_page_slice(storage, 0, 7, 2)
    );

    assert_eq!(
        vec![40, 50, 60, 70],
        helpers::storage_read_page_slice(storage, 1, 0, 4)
    );
}
