use std::ffi::c_void;

use wasmer_runtime::{func, imports, Func};

use svm_runtime::{
    helpers::DataWrapper,
    host_ctx::HostCtx,
    testing::{self, instance_register, instance_storage},
    vmcalls,
};

use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

fn default_test_args() -> (
    u32,
    u32,
    DataWrapper<*mut c_void>,
    DataWrapper<*const c_void>,
    u16,
) {
    let app_addr = 0x12_34_56_78;
    let state = 0x_00_00_00_00;
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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_reg_copy.wast"),
    );

    // we first initialize register `2:64` with some garbage data (0xFF...FF) which should be overriden
    // after calling the exported `do_copy_to_reg` function
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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
    // after calling the exported `do_copy_to_reg` function
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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
    let (app_addr, state, host, _, pages_count) = default_test_args();

    let mut host_ctx = HostCtx::new();
    host_ctx.insert(2, vec![10, 20]);
    host_ctx.insert(3, vec![30, 40, 50]);
    let host_ctx = DataWrapper::new(svm_common::into_raw(host_ctx));

    let import_object = imports! {
        move || testing::app_memory_state_creator(app_addr, state, host, host_ctx, pages_count),

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
