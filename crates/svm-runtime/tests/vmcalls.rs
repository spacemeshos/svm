use std::cell::Cell;

use wasmer_runtime::{func, imports, Func};
use wasmer_runtime_core::Module;

use svm_runtime::ctx_data_wrapper::SvmCtxDataWrapper;
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

use crate::helpers;
use svm_storage::helpers::wasmer_data_storage;

fn wasmer_compile(wasm: &[u8]) -> Module {
    let wasm = wabt::wat2wasm(&wasm).unwrap();
    svm_compiler::compile_program(&wasm).unwrap()
}

fn wasmer_compile_file(file: &str) -> Module {
    let wasm = include_str!(file);
    wasmer_compile(wasm)
}

#[test]
fn vmcalls_empty_wasm() {
    let wasm = r#"
        (module
          (func (export "do_nothing")))"#;

    let module = wasmer_compile(&wasm);
    let _instance = module.instantiate(&imports! {}).unwrap();
}

#[test]
fn vmcalls_mem_to_reg_copy() {
    let module = wasmer_compile_file("wasm/mem_to_reg_copy.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "mem_to_reg_copy" => func!(vmcalls::mem_to_reg_copy),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // initializing memory #0 cells `200..203` with values `10, 20, 30` respectively
    svm_runtime::wasmer_ctx_mem_cells_write!(instance.context(), 0, 200, &[10, 20, 30]);

    // asserting register `2` (of type `64 bits`) content is empty prior copy
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 2);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0], reg.view());

    let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
    assert!(do_copy.call(200, 3, 2).is_ok());

    // asserting register `2` (of type `64 bits`) content is `10, 20, 30, 0, ... 0`
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 2);
    assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_reg_to_mem_copy() {
    let module = wasmer_compile_file("wasm/reg_to_mem_copy.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "reg_to_mem_copy" => func!(vmcalls::reg_to_mem_copy),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // initializing reg `2` (of type `64 bits`) with values `10, 20, 30` respectively
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 2);
    reg.set(&[10, 20, 30]);

    // asserting memory #0, cells `0..3` are zeros before copy
    let cells = svm_runtime::wasmer_ctx_mem_cells!(instance.context(), 0, 0, 3);
    assert_eq!([Cell::new(0), Cell::new(0), Cell::new(0)], cells);

    // copying reg `2` content into memory cells `0..3`
    let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();
    assert!(do_copy.call(2, 3, 0).is_ok());

    // asserting memory #0, cells `0..3` have the values `10, 20, 30` respectively
    let cells = svm_runtime::wasmer_ctx_mem_cells!(instance.context(), 0, 0, 3);
    assert_eq!([Cell::new(10), Cell::new(20), Cell::new(30)], cells);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_reg() {
    let module = wasmer_compile_file("wasm/storage_to_reg_copy.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // we first initialize register `2` with some garbage data (0xFF...FF) which should be overriden
    // after calling the exported `do_copy_to_reg` function
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 2);
    reg.set(&[0xFF; 8]);
    assert_eq!(vec![0xFF; 8], reg.view());

    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
    assert!(do_copy.call(1, 100, 3, 2).is_ok());

    // register `2:64` should contain zeros, since an empty page-slice is treated as a page-slice containing only zeros
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 2);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_reg() {
    let module = wasmer_compile_file("wasm/storage_to_reg_copy.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = wasmer_data_storage(instance.context_mut().data);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    // we write `[10, 20, 30]` into storage slice (page `1`, cells: `100..103`)
    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    // we first initialize register `2:64` with some garbage (0xFF...FF) data which should be overriden
    // after calling the exported `do_copy_to_reg` function
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 2);
    reg.set(&[0xFF; 8]);
    assert_eq!(vec![0xFF; 8], reg.view());

    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();

    // we copy slice (page `1`, cells: `100..103`) into register `2:64`
    assert!(do_copy.call(1, 100, 3, 2).is_ok());

    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 2);
    assert_eq!(vec![10, 20, 30, 0, 0, 0, 0, 0], reg.view());
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_mem() {
    let module = wasmer_compile_file!("wasm/storage_to_mem_copy.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "storage_read_to_mem" => func!(vmcalls::storage_read_to_mem),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // we fill memory #0, cells `200..203` with garbage data (0xFF...FF)
    svm_runtime::wasmer_ctx_mem_cells_write!(instance.context(), 0, 200, &[0xFF, 0xFF, 0xFF]);
    let cells = svm_runtime::wasmer_ctx_mem_cells!(instance.context(), 0, 200, 3);
    assert_eq!(&[Cell::new(0xFF), Cell::new(0xFF), Cell::new(0xFF)], cells);

    // we copy storage slice (page `1`, cells: `100..103`) into memory starting from `address = 200`
    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();
    assert!(do_copy.call(1, 100, 3, 200).is_ok());

    let cells = svm_runtime::wasmer_ctx_mem_cells!(instance.context(), 0, 200, 3);
    assert_eq!(&[Cell::new(0), Cell::new(0), Cell::new(0)], cells);
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_mem() {
    let module = wasmer_compile_file!("wasm/storage_to_mem_copy.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "storage_read_to_mem" => func!(vmcalls::storage_read_to_mem),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = svm_runtime::wasmer_data_storage!(instance.context_mut().data);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    // we write `[10, 20, 30]` into storage slice (page `1`, cells `100..103`)
    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    let do_copy: Func<(i32, i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();

    // we copy slice (page `1`, cells: `100..103`) into memory #0, starting from address `200`
    assert!(do_copy.call(1, 100, 3, 200).is_ok());

    let cells = svm_runtime::wasmer_ctx_mem_cells!(instance.context(), 0, 200, 3);
    assert_eq!(&[Cell::new(10), Cell::new(20), Cell::new(30)], cells);
}

#[test]
fn vmcalls_storage_write_from_mem() {
    let module = wasmer_compile_file("wasm/storage_write_from_mem.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "storage_write_from_mem" => func!(vmcalls::storage_write_from_mem),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = svm_runtime::wasmer_data_storage!(instance.context_mut().data);

    svm_runtime::wasmer_ctx_mem_cells_write!(instance.context(), 0, 200, &[10, 20, 30]);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    assert_eq!(vec![0, 0, 0], storage.read_page_slice(&layout));

    let do_write: Func<(i32, i32, i32, i32)> = instance.func("do_write_from_mem").unwrap();

    // we copy memory cells `200..`203` into storage (`page 1`, cells: `100..103`)
    assert!(do_write.call(200, 3, 1, 100).is_ok());

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_storage_write_from_reg() {
    let module = wasmer_compile_file("wasm/storage_write_from_reg.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "storage_write_from_reg" => func!(vmcalls::storage_write_from_reg),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = svm_runtime::wasmer_data_storage!(instance.context_mut().data);

    // we first initialize register `5:64` with `[10, 20, 30, 0, 0, 0, 0, 0]`
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 5);
    reg.set(&[10, 20, 30]);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(200), 3);

    assert_eq!(vec![0, 0, 0], storage.read_page_slice(&layout));

    let do_write: Func<(i32, i32, i32, i32)> = instance.func("do_write_from_reg").unwrap();

    // we copy register `5:64` first into storage (`page 1`, cells: `200..203`)
    assert!(do_write.call(5, 3, 1, 200).is_ok());

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_reg_replace_byte_read_write_be_i64() {
    let module = wasmer_compile_file("wasm/reg_replace_read_write_be_i64.wast");

    let import_object = imports! {
        test_create_svm_state_gen!(),

        "svm" => {
            "storage_read_to_reg" => func!(vmcalls::storage_read_to_reg),
            "storage_write_from_reg" => func!(vmcalls::storage_write_from_reg),
            "reg_replace_byte" => func!(vmcalls::reg_replace_byte),
            "reg_read_be_i64" => func!(vmcalls::reg_read_be_i64),
            "reg_write_be_i64" => func!(vmcalls::reg_write_be_i64),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // we first initialize register `64:5` with `[254, 255, 0, 0, 0, 0, 0, 0]`
    let reg = svm_runtime::wasmer_ctx_reg!(instance.context(), 64, 5);
    reg.set(&[0, 0, 0, 0, 0, 0, 255, 254]);

    let inc: Func<i32> = instance.func("inc").unwrap();
    assert!(inc.call(5).is_ok());
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 255, 255], reg.view());

    assert!(inc.call(5).is_ok());
    assert_eq!(vec![0, 0, 0, 0, 0, 1, 0, 0], reg.view());

    // now we'll change 2 bytes of register `64:5`
    let replace: Func<(i32, i32, i32)> = instance.func("replace").unwrap();
    assert!(replace.call(5, 10, 6).is_ok());
    assert!(replace.call(5, 20, 7).is_ok());
    assert_eq!(vec![0, 0, 0, 0, 0, 1, 10, 20], reg.view());
}
