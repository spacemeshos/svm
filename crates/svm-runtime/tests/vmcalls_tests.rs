#![allow(unused)]
use maplit::hashmap;

use std::ffi::c_void;

use wasmer::{imports, Function, NativeFunc};

use svm_layout::{DataLayout, VarId};
use svm_runtime::{testing, vmcalls, Context};
use svm_types::{gas::MaybeGas, receipt::Log, Address, HostCtx};

macro_rules! assert_vars32 {
    ($instance:expr, $( $var_id:expr => $expected:expr), *) => {{
        __assert_vars_impl!(u32, $instance, $( $var_id => $expected), *)
    }};
}

macro_rules! assert_vars64 {
    ($instance:expr, $( $var_id:expr => $expected:expr), *) => {{
        __assert_vars_impl!(u64, $instance, $( $var_id => $expected), *)
    }};
}

macro_rules! __assert_vars_impl {
    ($ty:ty, $instance:expr, $( $var_id:expr => $expected:expr), *) => {{
        let func: &NativeFunc<u32, $ty> = &$instance.exports.get_native_function("get").unwrap();

        $( assert_eq!(func.call($var_id).unwrap(), $expected); )*
    }};
}

macro_rules! assert_storage {
    ($ctx:expr, $($var_id:expr => $expected:expr), *) => {{
        let storage = &$ctx.borrow().storage;

        $(
            let actual = storage.read_var(VarId($var_id));
            assert_eq!(actual, $expected);
         )*
    }};
}

macro_rules! var_add32 {
    ($instance:expr, $var_id:expr, $amount:expr) => {{
        __var_add_impl!(u32, $instance, $var_id, $amount)
    }};
}

macro_rules! var_add64 {
    ($instance:expr, $var_id:expr, $amount:expr) => {{
        __var_add_impl!(u64, $instance, $var_id, $amount)
    }};
}

macro_rules! __var_add_impl {
    ($ty:ty, $instance:expr, $var_id:expr, $amount:expr) => {{
        let func: NativeFunc<(u32, $ty), ()> =
            $instance.exports.get_native_function("add").unwrap();

        let res = func.call($var_id, $amount);

        assert!(res.is_ok());
    }};
}

macro_rules! host_ctx {
    ($($field:expr => $bytes:expr),*) => {{
        HostCtx::from(hashmap! {
            $( $field => $bytes.to_vec() ),*
        })
    }};
}

macro_rules! assert_host_ctx {
    ($instance:expr, $( $field:expr => $expected:expr), *) => {{
        let func: &NativeFunc<u32, u64> = &$instance.exports.get_native_function("get_host_ctx").unwrap();

        $( assert_eq!(func.call($field).unwrap(), $expected); )*
    }}
}

macro_rules! func {
    ($store:ident, $ctx:ident, $f:expr) => {{
        Function::new_native_with_env(&$store, $ctx.clone(), $f)
    }};
}

#[test]
fn vmcalls_empty_wasm() {
    let wasm = r#"
        (module
          (func (export "run")))"#
        .into();

    let gas_limit = MaybeGas::new();

    let store = testing::wasmer_store();
    let import_object = imports! {};

    testing::wasmer_instantiate(&store, &import_object, wasm, gas_limit);
}

#[test]
fn vmcalls_get32_set32() {
    let app_addr = Address::of("my-app");
    let host: *mut c_void = std::ptr::null_mut();
    let host_ctx = host_ctx! {};
    let gas_limit = MaybeGas::new();
    let layout: DataLayout = vec![4, 2].into();

    let store = testing::wasmer_store();
    let memory = testing::wasmer_memory(&store);
    let storage = testing::blank_storage(&app_addr, &layout);
    let ctx = Context::new(memory, host, host_ctx, gas_limit, storage);

    let import_object = imports! {
        "svm" => {
            "get32" => func!(store, ctx, vmcalls::get32),
            "set32" => func!(store, ctx, vmcalls::set32),
        }
    };

    let instance = testing::wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/get32_set32.wast").into(),
        gas_limit,
    );

    assert_vars32!(instance, 0 => 0, 1 => 0);

    var_add32!(instance, 0, 5); // adding 5 to var #0
    var_add32!(instance, 1, 10); // adding 10 to var #1

    assert_vars32!(instance, 0 => 5, 1 => 10);

    assert_storage!(ctx, 0 => [5, 0, 0, 0], 1 => [10, 0]);
}

#[test]
fn vmcalls_get64_set64() {
    let app_addr = Address::of("my-app");
    let host: *mut c_void = std::ptr::null_mut();
    let host_ctx = host_ctx! {};
    let gas_limit = MaybeGas::new();
    let layout: DataLayout = vec![4, 2].into();

    let store = testing::wasmer_store();
    let memory = testing::wasmer_memory(&store);
    let storage = testing::blank_storage(&app_addr, &layout);
    let ctx = Context::new(memory, host, host_ctx, gas_limit, storage);

    let import_object = imports! {
        "svm" => {
            "get64" => func!(store, ctx, vmcalls::get64),
            "set64" => func!(store, ctx, vmcalls::set64),
        },
    };

    let instance = testing::wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/get64_set64.wast").into(),
        gas_limit,
    );

    assert_vars64!(instance, 0 => 0, 1 => 0);

    var_add64!(instance, 0, 5); // adding 5 to var #0
    var_add64!(instance, 1, 10); // adding 10 to var #1

    assert_vars64!(instance, 0 => 5, 1 => 10);

    assert_storage!(ctx, 0 => [5, 0, 0, 0], 1 => [10, 0]);
}

#[test]
fn vmcalls_load160() {
    let app_addr = Address::of("11223344556677889900");
    let host: *mut c_void = std::ptr::null_mut();
    let gas_limit = MaybeGas::new();
    let layout: DataLayout = vec![20].into();

    let host_ctx = host_ctx! {};

    let store = testing::wasmer_store();
    let memory = testing::wasmer_memory(&store);
    let storage = testing::blank_storage(&app_addr, &layout);
    let ctx = Context::new(memory.clone(), host, host_ctx, gas_limit, storage);

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "load160" => func!(store, ctx, vmcalls::load160),
            "store160" => func!(store, ctx, vmcalls::store160),
        },
    };

    let instance = testing::wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/load160_store160.wast").into(),
        gas_limit,
    );

    {
        let storage = &mut ctx.borrow_mut().storage;
        storage.write_var(VarId(0), app_addr.as_slice().to_vec());
    }

    let func: NativeFunc<(u32, u32)> = instance.exports.get_native_function("load").unwrap();
    let ptr = 0;
    let var_id = 0;

    func.call(var_id, ptr).expect("function has failed");

    let view = &memory.view::<u8>()[ptr as usize..(ptr as usize + 20)];
    let bytes: Vec<u8> = view.iter().map(|cell| cell.get()).collect();

    assert_eq!(app_addr, Address::from(&bytes[..]));
}

#[test]
fn vmcalls_store160() {
    let app_addr = Address::of("11223344556677889900");
    let host: *mut c_void = std::ptr::null_mut();
    let gas_limit = MaybeGas::new();
    let layout: DataLayout = vec![20].into();

    let host_ctx = host_ctx! {};

    let store = testing::wasmer_store();
    let memory = testing::wasmer_memory(&store);
    let storage = testing::blank_storage(&app_addr, &layout);
    let ctx = Context::new(memory.clone(), host, host_ctx, gas_limit, storage);

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "load160" => func!(store, ctx, vmcalls::load160),
            "store160" => func!(store, ctx, vmcalls::store160),
        },
    };

    let instance = testing::wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/load160_store160.wast").into(),
        gas_limit,
    );

    for (cell, byte) in memory.view::<u8>().iter().zip(app_addr.as_slice()) {
        cell.set(*byte);
    }

    let func: NativeFunc<(u32, u32)> = instance.exports.get_native_function("store").unwrap();
    let ptr = 0;
    let var_id = 0;

    func.call(var_id, ptr).expect("function has failed");

    assert_storage!(ctx, 0 => b"11223344556677889900");
}

#[test]
fn vmcalls_host_get64() {
    let app_addr = Address::of("my-app");
    let host: *mut c_void = std::ptr::null_mut();
    let gas_limit = MaybeGas::new();
    let layout = DataLayout::empty();

    let host_ctx = host_ctx! {
        2 => [0x10, 0x20],
        3 => [0x30, 0x40, 0x50]
    };

    let store = testing::wasmer_store();
    let memory = testing::wasmer_memory(&store);
    let storage = testing::blank_storage(&app_addr, &layout);
    let ctx = Context::new(memory, host, host_ctx, gas_limit, storage);

    let import_object = imports! {
        "svm" => {
            "host_get64" => func!(store, ctx, vmcalls::host_get64),
        },
    };

    let instance = testing::wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/host_get64.wast").into(),
        gas_limit,
    );

    assert_host_ctx!(instance,
        2 => 0x20_10,
        3 => 0x50_40_30
    );
}

#[test]
fn vmcalls_log() {
    let app_addr = Address::of("my-app");
    let host: *mut c_void = std::ptr::null_mut();
    let gas_limit = MaybeGas::new();
    let layout = DataLayout::empty();

    let host_ctx = host_ctx! {};

    let store = testing::wasmer_store();
    let memory = testing::wasmer_memory(&store);
    let storage = testing::blank_storage(&app_addr, &layout);
    let ctx = Context::new(memory.clone(), host, host_ctx, gas_limit, storage);

    let import_object = imports! {
        "svm" => {
            "memory" => memory.clone(),
            "log" => func!(store, ctx, vmcalls::log),
        },
    };

    let instance = testing::wasmer_instantiate(
        &store,
        &import_object,
        include_str!("wasm/log.wast").into(),
        gas_limit,
    );

    let data = b"Hello World";

    for (cell, byte) in memory.view::<u8>().iter().zip(data) {
        cell.set(*byte);
    }

    let logs = ctx.borrow_mut().take_logs();
    assert!(logs.is_empty());

    let func = instance.exports.get_function("sayHello").unwrap();
    let _ = func.call(&[]).unwrap();

    let logs = ctx.borrow_mut().take_logs();

    assert_eq!(
        logs,
        vec![Log {
            msg: b"Hello World".to_vec(),
            code: 200
        }]
    );
}
