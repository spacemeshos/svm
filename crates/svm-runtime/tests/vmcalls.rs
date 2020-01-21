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
    let state = State::empty();
    let host = DataWrapper::new(std::ptr::null_mut());
    let host_ctx = DataWrapper::new(svm_common::into_raw(HostCtx::new()));
    let page_count = 5;

    (app_addr, state, host, host_ctx, page_count)
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
    let reg_bits = 128;
    let reg_idx = 2;
    let reg_size = reg_bits / reg_idx;
    let mem_offset = 200;
    let data = [10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "mem_to_reg_copy" => func!(vmcalls::mem_to_reg_copy),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/mem_to_reg_copy.wast"));

    // initializing memory #0 cells `200..203` with values `10, 20, 30` respectively
    testing::instance_memory_init(&instance, mem_offset, &data);

    // asserting register  content is initialized with zeros
    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(vec![0; reg_size as usize], reg.view());

    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(mem_offset, reg_bits, reg_idx, len).is_ok());

    // asserting register content is `10, 20, 30, 0, 0, ... 0`
    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(&data, &reg.view()[0..3]);
}

#[test]
fn vmcalls_reg_to_mem_copy() {
    let reg_bits = 128;
    let reg_idx = 2;
    let mem_offset = 200;
    let data = [10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "reg_to_mem_copy" => func!(vmcalls::reg_to_mem_copy),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/reg_to_mem_copy.wast"));

    // initializing register with values `10, 20, 30` respectively
    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&data);

    // asserting memory #0, cells `0..3` are zeros before copy
    let cells = testing::instance_memory_view(&instance, mem_offset, len);
    assert_eq!(vec![0; len as usize], cells);

    // copying register content into memory cells `0..3`
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(reg_bits, reg_idx, mem_offset, len).is_ok());

    // asserting memory #0, cells `0..3` have the values `10, 20, 30` respectively
    let cells = testing::instance_memory_view(&instance, mem_offset, len);
    assert_eq!(&data[..], &cells[..]);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_reg() {
    let reg_bits = 128;
    let reg_idx = 2;
    let reg_size = reg_bits / reg_idx;
    let page_idx = 1;
    let page_offset = 100;
    let data = [10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_reg_copy.wast"),
    );

    // we first initialize ther register with some garbage data (0xFF...FF) which should be overriden
    // after calling the exported `run` function
    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&vec![0xFF; reg_size as usize]);

    let func: Func<(i32, i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func
        .call(page_idx, page_offset, reg_bits, reg_idx, len)
        .is_ok());

    // register should contain zeros, since an empty page-slice is treated as a page-slice containing only zeros
    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(vec![0; reg_size as usize], reg.view());
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_reg() {
    let reg_bits = 128;
    let reg_idx = 2;
    let reg_size = reg_bits / reg_idx;
    let page_idx = 1;
    let page_offset = 100;
    let data = [10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_reg_copy.wast"),
    );

    let storage = instance_storage(&instance);
    let layout = PageSliceLayout::new(
        PageIndex(page_idx as u16),
        PageOffset(page_offset as u32),
        len as u32,
    );
    storage.write_page_slice(&layout, &data);

    // we first initialize register with some garbage (0xFF...FF) data which should be overriden
    // after calling the exported `run` function
    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&vec![0xFF; reg_size as usize]);

    // we copy slice into register
    let func: Func<(i32, i32, i32, i32, i32)> = instance.func("run").unwrap();

    assert!(func
        .call(page_idx, page_offset, reg_bits, reg_idx, len)
        .is_ok());

    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(&data[..], &reg.view()[0..len as usize]);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_mem() {
    let page_idx = 1;
    let page_offset = 200;
    let mem_offset = 100;
    let data = [10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "storage_read_to_mem" => func!(vmcalls::storage_read_to_mem),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_mem_copy.wast"),
    );

    // we fill memory #0, cells  with garbage data (0xFF...FF)
    testing::instance_memory_init(&instance, page_offset, &vec![0xFF; len as usize]);

    let cells = testing::instance_memory_view(&instance, page_offset, len);
    assert_eq!(vec![0xFF; len as usize], cells);

    // we copy page-slice into memory `#0`
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(page_idx, page_offset, mem_offset, len).is_ok());

    let cells = testing::instance_memory_view(&instance, mem_offset, len);
    assert_eq!(vec![0; len as usize], cells);
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_mem() {
    let page_idx = 1;
    let page_offset = 100;
    let mem_offset = 200;
    let data = vec![10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "storage_read_to_mem" => func!(vmcalls::storage_read_to_mem),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_to_mem_copy.wast"),
    );

    let storage = instance_storage(&instance);
    let layout = PageSliceLayout::new(
        PageIndex(page_idx as u16),
        PageOffset(page_offset as u32),
        len as u32,
    );
    storage.write_page_slice(&layout, &data[..]);

    // we copy slice (page `1`, cells: `100..103`) into memory #0, starting from address `200`
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(page_idx, page_offset, mem_offset, len).is_ok());

    let cells = testing::instance_memory_view(&instance, mem_offset, len);
    assert_eq!(data, cells);
}

#[test]
fn vmcalls_storage_write_from_mem() {
    let page_idx = 1;
    let page_offset = 100;
    let mem_offset = 200;
    let data = vec![10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "storage_write_from_mem" => func!(vmcalls::storage_write_from_mem),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_write_from_mem.wast"),
    );

    // initializing memory `#0` cells `200...203` with `10, 20, 30` respectively
    testing::instance_memory_init(&instance, mem_offset, &data[..]);

    let storage = instance_storage(&instance);
    let layout = PageSliceLayout::new(
        PageIndex(page_idx as u16),
        PageOffset(page_offset as u32),
        len as u32,
    );

    assert_eq!(vec![0; len as usize], storage.read_page_slice(&layout));

    // we copy memory cells `200..`203` into storage (`page 1`, cells: `100..103`)
    let func: Func<(i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func.call(mem_offset, page_idx, page_offset, len).is_ok());

    assert_eq!(data, storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_storage_write_from_reg() {
    let reg_bits = 128;
    let reg_idx = 5;
    let page_idx = 1;
    let page_offset = 100;
    let data = vec![10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "storage_write_from_reg" => func!(vmcalls::storage_write_from_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/storage_write_from_reg.wast"),
    );

    let storage = instance_storage(&instance);

    // we first initialize register with `data`
    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&data[..]);

    let layout = PageSliceLayout::new(
        PageIndex(page_idx as u16),
        PageOffset(page_offset as u32),
        len as u32,
    );
    assert_eq!(vec![0; len as usize], storage.read_page_slice(&layout));

    // we copy register first into storage
    let func: Func<(i32, i32, i32, i32, i32)> = instance.func("run").unwrap();
    assert!(func
        .call(reg_bits, reg_idx, page_idx, page_offset, len)
        .is_ok());

    assert_eq!(data, storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_host_ctx_read_into_reg() {
    let reg_bits = 128;
    let reg_idx = 3;
    let field_idx = 3;
    let data = vec![10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, _host_ctx, page_count) = default_test_args();

    let host_ctx = HostCtx::from(hashmap! {
        2 => vec![10, 20],
        3 => data.clone()
    });

    let host_ctx = DataWrapper::new(svm_common::into_raw(host_ctx));

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "host_ctx_read_into_reg" => func!(vmcalls::host_ctx_read_into_reg),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/host_ctx_read_into_reg.wast"),
    );

    let func: Func<(i32, i32, i32)> = instance.func("run").unwrap();

    // copying field #2 (content=`[10, 20]`) into register
    assert!(func.call(field_idx, reg_bits, reg_idx).is_ok());

    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(&data[..], &reg.view()[0..len as usize]);
}

#[test]
fn vmcalls_buffer_copy_to_storage() {
    let buf_id = 3;
    let buf_offset = 0;
    let page_idx = 1;
    let page_offset = 5;
    let data = vec![10, 20, 30];
    let len = data.len() as i32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "buffer_create" => func!(vmcalls::buffer_create),
            "buffer_kill" => func!(vmcalls::buffer_kill),
            "buffer_copy_to_storage" => func!(vmcalls::buffer_copy_to_storage),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/buffer.wast"));

    // create buffer
    let func: Func<i32> = instance.func("create").unwrap();
    assert!(func.call(buf_id).is_ok());

    let buf = instance_buffer(&instance, buf_id).unwrap();
    buf.write(&data);

    // copy buf slice into page
    let func: Func<(i32, i32, i32, i32, i32)> = instance.func("copy").unwrap();
    assert!(func
        .call(buf_id, buf_offset, page_idx, page_offset, len)
        .is_ok());

    // killing buffer
    assert!(instance_buffer(&instance, buf_id).is_some());

    let func: Func<i32> = instance.func("kill").unwrap();
    assert!(func.call(buf_id).is_ok());

    assert!(instance_buffer(&instance, buf_id).is_none());

    // asserting persisted storage
    let storage = instance_storage(&instance);
    assert_eq!(
        data,
        helpers::storage_read_page_slice(storage, page_idx, page_offset, len)
    );
}
