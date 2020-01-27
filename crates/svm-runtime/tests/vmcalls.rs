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
    let reg_size = reg_bits / 8;
    let mem_offset = 200;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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

    let func: Func<(u32, u32, u32, u32)> = instance.func("run").unwrap();
    assert!(func.call(mem_offset, reg_bits, reg_idx, count).is_ok());

    // asserting register content is `10, 20, 30, 0, 0, ... 0`
    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(&data[..], &reg.view()[0..count as usize]);
}

#[test]
fn vmcalls_reg_to_mem_copy() {
    let reg_bits = 128;
    let reg_idx = 2;
    let mem_offset = 200;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "reg_to_mem_copy" => func!(vmcalls::reg_to_mem_copy),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/reg_to_mem_copy.wast"));

    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&data[..]);

    let before = testing::instance_memory_view(&instance, mem_offset, count);
    assert_eq!(vec![0; count as usize], before);

    // copying register into memory
    let func: Func<(u32, u32, u32, u32)> = instance.func("run").unwrap();
    assert!(func.call(reg_bits, reg_idx, mem_offset, count).is_ok());

    let after = testing::instance_memory_view(&instance, mem_offset, count);
    assert_eq!(data, after);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_reg() {
    let reg_bits = 128;
    let reg_idx = 2;
    let reg_size = reg_bits / 8;
    let page_idx = 1;
    let page_offset = 100;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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

    let func: Func<(u32, u32, u32, u32, u32)> = instance.func("run").unwrap();
    assert!(func
        .call(page_idx, page_offset, reg_bits, reg_idx, count)
        .is_ok());

    // register should contain zeros, since an empty page-slice is treated as a page-slice containing only zeros
    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(vec![0; reg_size as usize], reg.view());
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_reg() {
    let reg_bits = 128;
    let reg_idx = 2;
    let reg_size = reg_bits / 8;
    let page_idx = 1;
    let page_offset = 100;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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
        count as u32,
    );
    storage.write_page_slice(&layout, &data);

    // we first initialize register with some garbage (0xFF...FF) data which should be overriden
    // after calling the exported `run` function
    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&vec![0xFF; reg_size as usize]);

    // we copy slice into register
    let func: Func<(u32, u32, u32, u32, u32)> = instance.func("run").unwrap();

    assert!(func
        .call(page_idx, page_offset, reg_bits, reg_idx, count)
        .is_ok());

    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(data, &reg.view()[0..count as usize]);
}

#[test]
fn vmcalls_storage_read_an_empty_page_slice_to_mem() {
    let page_idx = 1;
    let page_offset = 200;
    let mem_offset = 100;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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
    testing::instance_memory_init(&instance, page_offset, &vec![0xFF; count as usize]);

    let before = testing::instance_memory_view(&instance, page_offset, count);
    assert_eq!(vec![0xFF; count as usize], before);

    // we copy page-slice into memory `#0`
    let func: Func<(u32, u32, u32, u32)> = instance.func("run").unwrap();
    assert!(func.call(page_idx, page_offset, mem_offset, count).is_ok());

    let after = testing::instance_memory_view(&instance, mem_offset, count);
    assert_eq!(vec![0; count as usize], after);
}

#[test]
fn vmcalls_storage_read_non_empty_page_slice_to_mem() {
    let page_idx = 1;
    let page_offset = 100;
    let mem_offset = 200;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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
        count as u32,
    );
    storage.write_page_slice(&layout, &data[..]);

    // we copy slice (page `1`, cells: `100..103`) into memory #0, starting from address `200`
    let func: Func<(u32, u32, u32, u32)> = instance.func("run").unwrap();
    assert!(func.call(page_idx, page_offset, mem_offset, count).is_ok());

    let after = testing::instance_memory_view(&instance, mem_offset, count);
    assert_eq!(data, after);
}

#[test]
fn vmcalls_storage_write_from_mem() {
    let page_idx = 1;
    let page_offset = 100;
    let mem_offset = 200;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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
        count as u32,
    );

    let before = storage.read_page_slice(&layout);
    assert_eq!(vec![0; count as usize], before);

    // we copy memory cells `200..`203` into storage (`page 1`, cells: `100..103`)
    let func: Func<(u32, u32, u32, u32)> = instance.func("run").unwrap();
    assert!(func.call(mem_offset, page_idx, page_offset, count).is_ok());

    let after = storage.read_page_slice(&layout);
    assert_eq!(data, after);
}

#[test]
fn vmcalls_storage_write_from_reg() {
    let reg_bits = 128;
    let reg_idx = 5;
    let page_idx = 1;
    let page_offset = 100;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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
        count as u32,
    );

    let before = storage.read_page_slice(&layout);
    assert_eq!(vec![0; count as usize], before);

    // we copy register first into storage
    let func: Func<(u32, u32, u32, u32, u32)> = instance.func("run").unwrap();
    assert!(func
        .call(reg_bits, reg_idx, page_idx, page_offset, count)
        .is_ok());

    let after = storage.read_page_slice(&layout);
    assert_eq!(data, after);
}

#[test]
fn vmcalls_register_push() {
    let reg_bits = 128;
    let reg_idx = 3;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "reg_push" => func!(vmcalls::reg_push),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/reg_push.wast"));

    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&data[..]);

    // will call `reg_push` on input register
    let func: Func<(u32, u32)> = instance.func("run").unwrap();
    assert!(func.call(reg_bits, reg_idx).is_ok());

    let reg = instance_register(&instance, reg_bits, reg_idx);

    // we want to get back to where we were before doing `func.call(..)`
    reg.pop();

    assert_eq!(&data[..], &reg.view()[0..count as usize]);
}

#[test]
fn vmcalls_register_pop() {
    let reg_bits = 128;
    let reg_idx = 3;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

    let (app_addr, state, host, host_ctx, page_count) = default_test_args();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

        "svm" => {
            "reg_pop" => func!(vmcalls::reg_pop),
        },
    };

    let instance = testing::instantiate(&import_object, include_str!("wasm/reg_pop.wast"));

    let reg = instance_register(&instance, reg_bits, reg_idx);
    reg.set(&data[..]);
    reg.push();

    // will call `reg_pop` on input register
    let func: Func<(u32, u32)> = instance.func("run").unwrap();
    assert!(func.call(reg_bits, reg_idx).is_ok());

    // if `instance` triggered `reg_pop` we need to be back to where we were before calling `push`
    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(&data[..], &reg.view()[0..count as usize]);
}

#[test]
fn vmcalls_host_ctx_read_into_reg() {
    let reg_bits = 128;
    let reg_idx = 3;
    let field_idx = 3;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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

    let func: Func<(u32, u32, u32)> = instance.func("run").unwrap();

    // copying field #2 (content=`[10, 20]`) into register
    assert!(func.call(field_idx, reg_bits, reg_idx).is_ok());

    let reg = instance_register(&instance, reg_bits, reg_idx);
    assert_eq!(&data[..], &reg.view()[0..count as usize]);
}

#[test]
fn vmcalls_buffer_copy_to_storage() {
    let buf_id = 3;
    let buf_offset = 0;
    let page_idx = 1;
    let page_offset = 5;
    let data = vec![10, 20, 30];
    let count = data.len() as u32;

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
    let func: Func<u32> = instance.func("create").unwrap();
    assert!(func.call(buf_id).is_ok());

    let buf = instance_buffer(&instance, buf_id).unwrap();
    buf.write(&data);

    // copy buf slice into page
    let func: Func<(u32, u32, u32, u32, u32)> = instance.func("copy").unwrap();
    assert!(func
        .call(buf_id, buf_offset, page_idx, page_offset, count)
        .is_ok());

    // killing buffer
    assert!(instance_buffer(&instance, buf_id).is_some());

    let func: Func<u32> = instance.func("kill").unwrap();
    assert!(func.call(buf_id).is_ok());

    assert!(instance_buffer(&instance, buf_id).is_none());

    // asserting persisted storage
    let storage = instance_storage(&instance);
    assert_eq!(
        data,
        helpers::storage_read_page_slice(storage, page_idx, page_offset, count)
    );
}

macro_rules! assert_int_slice {
    ($expected:expr, $func:expr, $page_idx:expr, $page_offset:expr, $count:expr, $endianness:expr) => {{
        let actual = $func
            .call($page_idx, $page_offset, $count, $endianness)
            .unwrap();

        assert_eq!($expected, actual);
    }};
}

macro_rules! assert_i32_field {
    ($expected:expr, $instance:expr, $field_idx:expr, $endianness:expr) => {{
        let func: Func<(u32, u32), u32> = $instance.func("read_i32").unwrap();

        assert_eq!($expected, func.call($field_idx, $endianness).unwrap());
    }};
}

macro_rules! assert_i64_field {
    ($expected:expr, $instance:expr, $field_idx:expr, $endianness:expr) => {{
        let func: Func<(u32, u32), u64> = $instance.func("read_i64").unwrap();

        assert_eq!($expected, func.call($field_idx, $endianness).unwrap());
    }};
}

macro_rules! test_storage_read_int {
    ($slice_idx:expr, $endianness:expr, $expected:expr) => {{
	let expected: u64 = $expected;

	let slices = vec![
	    (0, 0, vec![0x10]),
	    (0, 1, vec![0x10, 0x20]),
	    (0, 3, vec![0x10, 0x20, 0x30]),
	    (0, 6, vec![0x10, 0x20, 0x30, 0x40]),
	    (1, 0, vec![0x10, 0x20, 0x30, 0x40, 0x50]),
	    (1, 5, vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60]),
	    (2, 0, vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70]),
	    (2, 7, vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]),
	];

	let (app_addr, state, host, host_ctx, page_count) = default_test_args();

	let import_object = imports! {
	    move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

	    "svm" => {
		"storage_read_i32_be" => func!(vmcalls::storage_read_i32_be),
		"storage_read_i32_le" => func!(vmcalls::storage_read_i32_le),
		"storage_read_i64_be" => func!(vmcalls::storage_read_i64_be),
		"storage_read_i64_le" => func!(vmcalls::storage_read_i64_le),
	    },
	};

	let instance = testing::instantiate(&import_object, include_str!("wasm/storage_read_int.wast"));
	let storage = instance_storage(&instance);

	// initialzing the app storage with the above `slices`
	for (page_idx, page_offset, data) in slices.iter() {
	    let layout = PageSliceLayout::new(
		PageIndex(*page_idx as u16),
		PageOffset(*page_offset as u32),
		data.len() as u32,
	    );
	    storage.write_page_slice(&layout, &data[..]);
	}

	let slice = &slices[$slice_idx];
	let (page_idx, page_offset, count) = (slice.0, slice.1, slice.2.len() as u32);

	if count <= 4 {
	    let func: Func<(u32, u32, u32, u32), u32> = instance.func("read_i32").unwrap();

	    assert_int_slice!(expected as u32, func, page_idx, page_offset, count, $endianness);
	}
	else {
	    let func: Func<(u32, u32, u32, u32), u64> = instance.func("read_i64").unwrap();

	    assert_int_slice!(expected, func, page_idx, page_offset, count, $endianness);
	}
    }};
}

#[test]
fn vmcalls_storage_read_int_32() {
    let be = 1; // Big-Endian
    let le = 0; // Little-Endian

    test_storage_read_int!(0, be, 0x10);
    test_storage_read_int!(0, le, 0x10);

    test_storage_read_int!(1, be, 0x10_20);
    test_storage_read_int!(1, le, 0x20_10);

    test_storage_read_int!(2, be, 0x10_20_30);
    test_storage_read_int!(2, le, 0x30_20_10);

    test_storage_read_int!(3, be, 0x10_20_30_40);
    test_storage_read_int!(3, le, 0x40_30_20_10);
}

#[test]
fn vmcalls_storage_read_int_64() {
    let be = 1; // Big-Endian
    let le = 0; // Little-Endian

    test_storage_read_int!(4, be, 0x10_20_30_40_50);
    test_storage_read_int!(4, le, 0x50_40_30_20_10);

    test_storage_read_int!(5, be, 0x10_20_30_40_50_60);
    test_storage_read_int!(5, le, 0x60_50_40_30_20_10);

    test_storage_read_int!(6, be, 0x10_20_30_40_50_60_70);
    test_storage_read_int!(6, le, 0x70_60_50_40_30_20_10);

    test_storage_read_int!(7, be, 0x10_20_30_40_50_60_70_80);
    test_storage_read_int!(7, le, 0x80_70_60_50_40_30_20_10);
}

macro_rules! test_host_ctx_read_int {
    ($field_idx:expr, $endianness:expr, $expected:expr) => {{
	let expected: u64 = $expected;

	let (app_addr, state, host, _host_ctx, page_count) = default_test_args();

	let fields = hashmap! {
	    0 => vec![0x10],
	    1 => vec![0x10, 0x20],
	    2 => vec![0x10, 0x20, 0x30],
	    3 => vec![0x10, 0x20, 0x30, 0x40],
	    4 => vec![0x10, 0x20, 0x30, 0x40, 0x50],
	    5 => vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60],
	    6 => vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70],
	    7 => vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]
	};

	let host_ctx = HostCtx::from(fields.clone());
	let host_ctx = DataWrapper::new(svm_common::into_raw(host_ctx));

	let import_object = imports! {
	    move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, page_count),

	    "svm" => {
		"host_ctx_read_i32_be" => func!(vmcalls::host_ctx_read_i32_be),
		"host_ctx_read_i32_le" => func!(vmcalls::host_ctx_read_i32_le),
		"host_ctx_read_i64_be" => func!(vmcalls::host_ctx_read_i64_be),
		"host_ctx_read_i64_le" => func!(vmcalls::host_ctx_read_i64_le),
	    },
	};

	let instance =
	    testing::instantiate(&import_object, include_str!("wasm/host_ctx_read_int.wast"));

	let field = fields.get(&$field_idx).unwrap().to_vec();
	let count = field.len();

	if count <= 4 {
	    assert_i32_field!(expected as u32, instance, $field_idx, $endianness);
	}
	else {
	    assert_i64_field!(expected, instance, $field_idx, $endianness);
	}
    }};
}

#[test]
fn vmcalls_host_ctx_read_int_32() {
    let be = 1; // Big-Endian
    let le = 0; // Little-Endian

    test_host_ctx_read_int!(0, be, 0x10);
    test_host_ctx_read_int!(0, le, 0x10);

    test_host_ctx_read_int!(1, be, 0x10_20);
    test_host_ctx_read_int!(1, le, 0x20_10);

    test_host_ctx_read_int!(2, be, 0x10_20_30);
    test_host_ctx_read_int!(2, le, 0x30_20_10);

    test_host_ctx_read_int!(3, be, 0x10_20_30_40);
    test_host_ctx_read_int!(3, le, 0x40_30_20_10);
}

#[test]
fn vmcalls_host_ctx_read_int_64() {
    let be = 1; // Big-Endian
    let le = 0; // Little-Endian

    test_host_ctx_read_int!(4, be, 0x10_20_30_40_50);
    test_host_ctx_read_int!(4, le, 0x50_40_30_20_10);

    test_host_ctx_read_int!(5, be, 0x10_20_30_40_50_60);
    test_host_ctx_read_int!(5, le, 0x60_50_40_30_20_10);

    test_host_ctx_read_int!(6, be, 0x10_20_30_40_50_60_70);
    test_host_ctx_read_int!(6, le, 0x70_60_50_40_30_20_10);

    test_host_ctx_read_int!(7, be, 0x10_20_30_40_50_60_70_80);
    test_host_ctx_read_int!(7, le, 0x80_70_60_50_40_30_20_10);
}
