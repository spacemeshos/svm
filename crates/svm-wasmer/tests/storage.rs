extern crate svm_wasmer;

use svm_wasmer::*;

use std::cell::Cell;
use std::ffi::c_void;

use svm_wasmer::ctx::SvmCtx;

use svm_common::Address;
use svm_storage::{
    default::DefaultPageCache,
    memory::{MemKVStore, MemPages},
    page::PageSliceLayout,
    PageSliceCache,
};

use wasmer_runtime::{func, imports, Func};

pub type MemPageCache<'pc, K = [u8; 32]> = DefaultPageCache<'pc, MemPages<K>>;

// injecting the `wasmer svm storage vmcalls` implemented with `MemPageCache` as the `PageCache` type
include_wasmer_svm_storage_vmcalls!(MemPageCache);

macro_rules! include_wasm_file {
    ($file:expr) => {{
        let bytes = include_bytes!($file);
        String::from_utf8_lossy(bytes).into_owned()
    }};
}

macro_rules! wasmer_compile_module {
    ($file:expr) => {{
        let wasm = include_wasm_file!($file);
        let wasm = wabt::wat2wasm(&wasm).unwrap();

        wasmer_runtime::compile(&wasm).unwrap()
    }};
}

#[test]
fn vmcalls_mem_to_reg_copy() {
    let module = wasmer_compile_module!("wasm/mem_to_reg_copy.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "mem_to_reg_copy" => func!(mem_to_reg_copy),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // initializing memory #0 cells `200..203` with values `10, 20, 30` respectively
    wasmer_ctx_mem_cells_write!(instance.context(), 0, 200, &[10, 20, 30]);

    // asserting register content is empty prior copy
    let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0], reg.get());

    let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
    assert!(do_copy.call(200, 3, 2).is_ok());

    // asserting register content is `10, 20, 30, 0, ... 0`
    let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
    assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg.get());
}

#[test]
fn vmcalls_reg_to_mem_copy() {
    let module = wasmer_compile_module!("wasm/reg_to_mem_copy.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "reg_to_mem_copy" => func!(reg_to_mem_copy),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();

    // initializing reg `2` with values `10, 20, 30` respectively
    wasmer_ctx_reg_write!(instance.context_mut(), 2, &[10, 20, 30], MemPageCache);

    // asserting memory #0, cells `0..3` are zeros before copy
    let cells = wasmer_ctx_mem_cells!(instance.context(), 0, 0, 3);
    assert_eq!([Cell::new(0), Cell::new(0), Cell::new(0)], cells);

    // copying reg `2` content into memory cells `0..3`
    let do_copy: Func<(i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();
    assert!(do_copy.call(2, 3, 0).is_ok());

    // asserting memory #0, cells `0..3` have the values `10, 20, 30` respectively
    let cells = wasmer_ctx_mem_cells!(instance.context(), 0, 0, 3);
    assert_eq!([Cell::new(10), Cell::new(20), Cell::new(30)], cells);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_reg() {
    let module = wasmer_compile_module!("wasm/storage_to_reg_copy.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "storage_read_to_reg" => func!(storage_read_to_reg),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // we first initialize register `2` with some garbage data which should be overriden
    // after calling the exported `do_copy_to_reg` function
    let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
    reg.set(&[255; 8]);

    assert_eq!([255; 8], reg.get());

    let do_copy: Func<(i32, i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
    assert!(do_copy.call(1, 10, 100, 3, 2).is_ok());

    // register `2` should contain zeros, since an empty page-slice is treated as a page-slice containing only zeros
    let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0], reg.get());
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_reg() {
    let module = wasmer_compile_module!("wasm/storage_to_reg_copy.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "storage_read_to_reg" => func!(storage_read_to_reg),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = wasmer_data_storage!(instance.context_mut().data, MemPageCache);
    let layout = svm_page_slice_layout!(1, 10, 100, 3);

    // we write `[10, 20, 30]` into storage slice `10` (page `1`, cells: `100..103`)
    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    // we first initialize register `2` with some garbage data which should be overriden
    // after calling the exported `do_copy_to_reg` function
    let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
    reg.set(&[255; 8]);

    let do_copy: Func<(i32, i32, i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();

    // we copy storage `slice 0` (page `1`, cells: `100..103`) into register `2`
    assert!(do_copy.call(1, 10, 100, 3, 2).is_ok());

    let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache);
    assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg.get());
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_mem() {
    let module = wasmer_compile_module!("wasm/storage_to_mem_copy.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "storage_read_to_mem" => func!(storage_read_to_mem),
        },
    };

    let instance = module.instantiate(&import_object).unwrap();

    // we fill memory #0, cells `200..203` with garbage data
    wasmer_ctx_mem_cells_write!(instance.context(), 0, 200, &[255, 255, 255]);
    let cells = wasmer_ctx_mem_cells!(instance.context(), 0, 200, 3);
    assert_eq!(&[Cell::new(255), Cell::new(255), Cell::new(255)], cells);

    // we copy storage `slice 0` (page `1`, cells: `100..103`) into memory starting from address = 200
    let do_copy: Func<(i32, i32, i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();
    assert!(do_copy.call(1, 10, 100, 3, 200).is_ok());

    let cells = wasmer_ctx_mem_cells!(instance.context(), 0, 200, 3);
    assert_eq!(&[Cell::new(0), Cell::new(0), Cell::new(0)], cells);
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_mem() {
    let module = wasmer_compile_module!("wasm/storage_to_mem_copy.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "storage_read_to_mem" => func!(storage_read_to_mem),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = wasmer_data_storage!(instance.context_mut().data, MemPageCache);
    let layout = svm_page_slice_layout!(1, 10, 100, 3);

    // we write `[10, 20, 30]` into storage slice `10` (page `1`, cells `100..103`)
    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    let do_copy: Func<(i32, i32, i32, i32, i32)> = instance.func("do_copy_to_mem").unwrap();

    // we copy storage `slice 0` (page `1`, cells: `100..103`) into memory #0, starting from address `200`
    assert!(do_copy.call(1, 10, 100, 3, 200).is_ok());

    let cells = wasmer_ctx_mem_cells!(instance.context(), 0, 200, 3);
    assert_eq!(&[Cell::new(10), Cell::new(20), Cell::new(30)], cells);
}

#[test]
fn vmcalls_storage_write_from_mem() {
    let module = wasmer_compile_module!("wasm/storage_write_from_mem.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "storage_write_from_mem" => func!(storage_write_from_mem),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = wasmer_data_storage!(instance.context_mut().data, MemPageCache);

    wasmer_ctx_mem_cells_write!(instance.context(), 0, 200, &[10, 20, 30]);

    let layout = svm_page_slice_layout!(1, 10, 100, 3);

    assert_eq!(None, storage.read_page_slice(&layout));

    let do_write: Func<(i32, i32, i32, i32, i32)> = instance.func("do_write_from_mem").unwrap();

    // we copy memory cells `200..`203` into storage (`page 1`, `slice 10`, cells: `100..103`)
    assert!(do_write.call(200, 3, 1, 10, 100).is_ok());

    assert_eq!(Some(vec![10, 20, 30]), storage.read_page_slice(&layout));
}

#[test]
fn vmcalls_storage_write_from_reg() {
    let module = wasmer_compile_module!("wasm/storage_write_from_reg.wast");

    let import_object = imports! {
        lazy_create_svm_import_object!(0x12_34_56_78, MemKVStore, MemPages, MemPageCache, 5, 100),

        "svm" => {
            "storage_write_from_reg" => func!(storage_write_from_reg),
        },
    };

    let mut instance = module.instantiate(&import_object).unwrap();
    let storage = wasmer_data_storage!(instance.context_mut().data, MemPageCache);

    // we first initialize register `5` with `[10, 20, 30, 0, 0, 0, 0, 0]`
    let reg = wasmer_ctx_reg!(instance.context(), 5, MemPageCache);
    reg.set(&[10, 20, 30]);

    let layout = svm_page_slice_layout!(1, 10, 100, 3);

    assert_eq!(None, storage.read_page_slice(&layout));

    let do_write: Func<(i32, i32, i32, i32, i32)> = instance.func("do_write_from_reg").unwrap();

    // we copy register `5` first 3 bytes into storage (`page 1`, `slice 10`, cells: `200..203`)
    assert!(do_write.call(5, 3, 1, 10, 200).is_ok());

    assert_eq!(Some(vec![10, 20, 30]), storage.read_page_slice(&layout));
}
