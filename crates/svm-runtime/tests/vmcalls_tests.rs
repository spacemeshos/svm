use maplit::hashmap;
use wasmer_runtime::{func, imports, Func};

use svm_layout::DataLayout;
use svm_runtime::{
    helpers::DataWrapper,
    testing::{self, instance_storage},
    vmcalls,
};
use svm_types::{gas::MaybeGas, Address, HostCtx};

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
        let ctx = HostCtx::from(hashmap! {
            $( $field => $bytes.to_vec() ),*
        });

        DataWrapper::new(svm_common::into_raw(ctx))
    }};
}

macro_rules! assert_host_ctx {
    ($instance:expr, $( $field:expr => $expected:expr), *) => {{
        let func: Func<u32, u64> = $instance.exports.get("get_host_ctx").unwrap();

        $( assert_eq!(func.call($field), Ok($expected)); )*
    }}
}

#[test]
fn vmcalls_empty_wasm() {
    let wasm = r#"
        (module
          (func (export "run")))"#;

    let maybe_gas = MaybeGas::new();

    testing::instantiate(&imports! {}, wasm, maybe_gas);
}

#[test]
fn vmcalls_get32_set32() {
    let app_addr = Address::of("my-app");
    let host = DataWrapper::new(std::ptr::null_mut());
    let host_ctx = host_ctx! {};
    let maybe_gas = MaybeGas::new();
    let layout: DataLayout = vec![4, 2].into();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, host, host_ctx, maybe_gas, &layout),

        "svm" => {
            "get32" => func!(vmcalls::get32),
            "set32" => func!(vmcalls::set32),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/get32_set32.wast"),
        maybe_gas,
    );

    assert_vars32!(instance, 0 => 0, 1 => 0);

    var_add32!(instance, 0, 5); // adding 5 to var #0
    var_add32!(instance, 1, 10); // adding 10 to var #1

    assert_vars32!(instance, 0 => 5, 1 => 10);
    assert_storage!(instance, 0 => [5, 0, 0, 0], 1 => [10, 0]);
}

#[test]
fn vmcalls_get64_set64() {
    let app_addr = Address::of("my-app");
    let host = DataWrapper::new(std::ptr::null_mut());
    let host_ctx = host_ctx! {};
    let maybe_gas = MaybeGas::new();
    let layout: DataLayout = vec![4, 2].into();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, host, host_ctx, maybe_gas, &layout),

        "svm" => {
            "get64" => func!(vmcalls::get64),
            "set64" => func!(vmcalls::set64),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/get64_set64.wast"),
        maybe_gas,
    );

    assert_vars64!(instance, 0 => 0, 1 => 0);

    var_add64!(instance, 0, 5); // adding 5 to var #0
    var_add64!(instance, 1, 10); // adding 10 to var #1

    assert_vars64!(instance, 0 => 5, 1 => 10);
    assert_storage!(instance, 0 => [5, 0, 0, 0], 1 => [10, 0]);
}

#[test]
fn host_get64() {
    let app_addr = Address::of("my-app");
    let host = DataWrapper::new(std::ptr::null_mut());
    let maybe_gas = MaybeGas::new();
    let layout = DataLayout::empty();

    let host_ctx = host_ctx! {
        2 => [0x10, 0x20],
        3 => [0x30, 0x40, 0x50]
    };

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, host, host_ctx, maybe_gas, &layout),

        "svm" => {
            "host_get64" => func!(vmcalls::host_get64),
        },
    };

    let instance = testing::instantiate(
        &import_object,
        include_str!("wasm/host_get64.wast"),
        maybe_gas,
    );

    assert_host_ctx!(instance,
        2 => 0x20_10,
        3 => 0x50_40_30
    );
}
