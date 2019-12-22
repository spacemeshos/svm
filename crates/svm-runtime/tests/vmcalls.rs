use wasmer_runtime::{func, imports, Func};

use crate::testing::{instance_register, instance_storage};
use svm_runtime::{helpers::PtrWrapper, testing, vmcalls};
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

fn prepare_test_args() -> (u32, u32, PtrWrapper, u32) {
    let addr = 0x12_34_56_78;
    let state = 0x_00_00_00_00;
    let host = PtrWrapper::new(std::ptr::null());
    let pages_count = 5;

    (addr, state, host, pages_count)
}

#[test]
fn vmcalls_empty_wasm() {
    let wasm = r#"
        (module
          (func (export "do_nothing")))"#;

    testing::instantiate(&imports! {}, wasm);
}

#[test]
fn vmcalls_mem_to_reg_copy() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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

    let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
    assert!(do_copy.call(200, 3, 2).is_ok());

    // asserting register `64:2` content is `10, 20, 30, 0, 0, ... 0`
    let reg = instance_register(&instance, 64, 2);
    assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_reg_to_mem_copy() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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
    let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();
    assert!(do_copy.call(2, 3, 0).is_ok());

    // asserting memory #0, cells `0..3` have the values `10, 20, 30` respectively
    let cells = testing::instance_memory_view(&instance, 0, 3);
    assert_eq!(vec![10, 20, 30], cells);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_reg() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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

    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
    assert!(do_copy.call(1, 100, 3, 2).is_ok());

    // register `64:2` should contain zeros, since an empty page-slice is treated as a page-slice containing only zeros
    let reg = instance_register(&instance, 64, 2);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_reg() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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
    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
    assert!(do_copy.call(1, 100, 3, 2).is_ok());

    let reg = instance_register(&instance, 64, 2);
    assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_mem() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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
    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();
    assert!(do_copy.call(1, 100, 3, 200).is_ok());

    let cells = testing::instance_memory_view(&instance, 200, 3);
    assert_eq!(vec![0, 0, 0], cells);
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_mem() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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

    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();

    // we copy slice (page `1`, cells: `100..103`) into memory #0, starting from address `200`
    assert!(do_copy.call(1, 100, 3, 200).is_ok());

    let cells = testing::instance_memory_view(&instance, 200, 3);
    assert_eq!(vec![10, 20, 30], cells);
}

#[test]
fn vmcalls_storage_write_from_mem() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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
    let do_write: Func<(i32, i32, i32, i32)> = instance.func("do_write_from_mem").unwrap();
    assert!(do_write.call(200, 3, 1, 100).is_ok());

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_storage_write_from_reg() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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
    let do_write: Func<(i32, i32, i32, i32)> = instance.func("do_write_from_reg").unwrap();
    assert!(do_write.call(5, 3, 1, 200).is_ok());

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_reg_replace_byte_read_write_be_i64() {
    let (addr, state, host, pages_count) = prepare_test_args();

    let import_object = imports! {
        move || testing::contract_memory_state_creator(addr, state, host, pages_count),

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

    let inc: Func<i32> = instance.func("inc").unwrap();
    assert!(inc.call(5).is_ok());

    let reg = instance_register(&instance, 64, 5);
    assert_eq!(vec![0, 0, 0, 0, 0, 1, 0, 0], reg.view());

    // now we'll change 2 bytes of register `64:5`
    let replace: Func<(i32, i32, i32)> = instance.func("replace").unwrap();
    assert!(replace.call(5, 10, 6).is_ok());
    assert!(replace.call(5, 20, 7).is_ok());

    let reg = instance_register(&instance, 64, 5);
    assert_eq!(vec![0, 0, 0, 0, 0, 1, 10, 20], reg.view());
}
