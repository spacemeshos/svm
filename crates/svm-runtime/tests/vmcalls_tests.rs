#![allow(unused)]
use maplit::hashmap;

use std::ffi::c_void;

use wasmer::imports;

use svm_layout::DataLayout;
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
        let func: Func<u32, $ty> = $instance.exports.get("get").unwrap();

        $( assert_eq!(func.call($var_id), Ok($expected)); )*
    }}
}

macro_rules! assert_storage {
    ($instance:expr, $($var_id:expr => $expected:expr), *) => {{
        use svm_layout::VarId;

        let storage = instance_storage(&$instance);

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
        let func: Func<(u32, $ty), ()> = $instance.exports.get("add").unwrap();
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
        let func: Func<u32, u64> = $instance.exports.get("get_host_ctx").unwrap();

        $( assert_eq!(func.call($field), Ok($expected)); )*
    }}
}

macro_rules! func {
    ($store:ident, $ctx:ident, $f:expr) => {{
        use wasmer::Function;

        Function::new_native_with_env(&$store, $ctx.clone(), $f)
    }};
}

#[test]
fn vmcalls_empty_wasm() {
    let wasm = r#"
        (module
          (func (export "run")))"#
        .into();

    let maybe_gas = MaybeGas::new();

    let store = testing::wasmer_store();
    let import_object = imports! {};

    testing::instantiate(&store, &import_object, wasm, maybe_gas);
}

#[test]
fn vmcalls_get32_set32() {
    let app_addr = Address::of("my-app");
    let host: *mut c_void = std::ptr::null_mut();
    let host_ctx = host_ctx! {};
    let maybe_gas = MaybeGas::new();
    let layout: DataLayout = vec![4, 2].into();

    let store = testing::wasmer_store();
    let memory = testing::wasmer_memory(&store);
    // let ctx = Context::new();

    // let import_object = imports! {
    //     "svm" => {
    //         "get32" => func!(store, ctx, vmcalls::get32),
    //         "set32" => func!(store, ctx, vmcalls::set32),
    //     }
    // };

    // let instance = testing::instantiate(
    //     &import_object,
    //     include_str!("wasm/get32_set32.wast"),
    //     maybe_gas,
    // );

    // assert_vars32!(instance, 0 => 0, 1 => 0);

    // var_add32!(instance, 0, 5); // adding 5 to var #0
    // var_add32!(instance, 1, 10); // adding 10 to var #1

    // assert_vars32!(instance, 0 => 5, 1 => 10);
    // assert_storage!(instance, 0 => [5, 0, 0, 0], 1 => [10, 0]);
}

// #[test]
// fn vmcalls_get64_set64() {
//     let app_addr = Address::of("my-app");
//     let host = DataWrapper::new(std::ptr::null_mut());
//     let host_ctx = host_ctx! {};
//     let maybe_gas = MaybeGas::new();
//     let layout: DataLayout = vec![4, 2].into();

//     let import_object = imports! {
//         "svm" => {
//             "get64" => func!(vmcalls::get64),
//             "set64" => func!(vmcalls::set64),
//         },
//     };

//     let instance = testing::instantiate(
//         &import_object,
//         include_str!("wasm/get64_set64.wast"),
//         maybe_gas,
//     );

//     assert_vars64!(instance, 0 => 0, 1 => 0);

//     var_add64!(instance, 0, 5); // adding 5 to var #0
//     var_add64!(instance, 1, 10); // adding 10 to var #1

//     assert_vars64!(instance, 0 => 5, 1 => 10);
//     assert_storage!(instance, 0 => [5, 0, 0, 0], 1 => [10, 0]);
// }

// #[test]
// fn vmcalls_host_get64() {
//     let app_addr = Address::of("my-app");
//     let host = DataWrapper::new(std::ptr::null_mut());
//     let maybe_gas = MaybeGas::new();
//     let layout = DataLayout::empty();

//     let host_ctx = host_ctx! {
//         2 => [0x10, 0x20],
//         3 => [0x30, 0x40, 0x50]
//     };

//     let import_object = imports! {
//         "svm" => {
//             "host_get64" => func!(vmcalls::host_get64),
//         },
//     };

//     let instance = testing::instantiate(
//         &import_object,
//         include_str!("wasm/host_get64.wast"),
//         maybe_gas,
//     );

//     assert_host_ctx!(instance,
//         2 => 0x20_10,
//         3 => 0x50_40_30
//     );
// }

// #[test]
// fn vmcalls_log() {
//     let app_addr = Address::of("my-app");
//     let host = DataWrapper::new(std::ptr::null_mut());
//     let maybe_gas = MaybeGas::new();
//     let layout = DataLayout::empty();

//     let host_ctx = host_ctx! {};

//     let minimum = Pages(1);
//     let maximum = None;
//     let shared = false;
//     let desc = MemoryDescriptor::new(minimum, maximum, shared).unwrap();
//     let memory = Memory::new(desc).unwrap();

//     let import_object = imports! {
//         "svm" => {
//             "memory" => memory,

//             "log" => func!(vmcalls::log),
//         },
//     };

//     let instance = testing::instantiate(&import_object, include_str!("wasm/log.wast"), maybe_gas);
//     let memory: &Memory = instance.context().memory(0);

//     let data = b"Hello World";

//     for (cell, byte) in memory.view::<u8>().iter().zip(data) {
//         cell.set(*byte);
//     }

//     let logs = testing::instance_logs(&instance);
//     assert!(logs.is_empty());

//     let func: Func = instance.exports.get("sayHello").unwrap();
//     let _ = func.call().unwrap();

//     let logs = testing::instance_logs(&instance);

//     assert_eq!(
//         logs,
//         vec![Log {
//             msg: b"Hello World".to_vec(),
//             code: 200
//         }]
//     );
// }
