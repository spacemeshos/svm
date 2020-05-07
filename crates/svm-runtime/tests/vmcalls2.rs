use wasmer_runtime::{func, imports, Func};

use svm_app::types::HostCtx;
use svm_common::{Address, State};
use svm_runtime::{
    gas::MaybeGas,
    helpers::DataWrapper,
    testing::{self, instance_storage2},
    vmcalls,
};
use svm_storage2::layout::DataLayout;

macro_rules! assert_vars {
    ($instance:expr, $( $var_id:expr => $expected:expr), *) => {{
        let func: Func<u32, u64> = $instance.func("get").unwrap();

        $( assert_eq!(func.call($var_id), Ok($expected)); )*
    }}
}

macro_rules! assert_storage {
    ($instance:expr, $($var_id:expr => $expected:expr), *) => {{
        use svm_storage2::layout::VarId;

        let storage = instance_storage2(&$instance);

        $(
            let actual = storage.read_var(VarId($var_id));
            assert_eq!(actual, $expected);
         )*
    }};
}

macro_rules! var_add {
    ($instance:expr, $var_id:expr, $amount:expr) => {{
        let func: Func<(u32, u64), ()> = $instance.func("add").unwrap();
        let res = func.call($var_id, $amount);

        assert!(res.is_ok());
    }};
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
fn vmcalls_get64_set64() {
    let app_addr = Address::of("my-app");
    let state = State::empty();
    let host = DataWrapper::new(std::ptr::null_mut());
    let host_ctx = DataWrapper::new(svm_common::into_raw(HostCtx::new()));
    let maybe_gas = MaybeGas::new();
    let page_count = 0;

    let layout: DataLayout = vec![4, 2].into();

    let import_object = imports! {
        move || testing::app_memory_state_creator(&app_addr, &state, host, host_ctx, maybe_gas, page_count, &layout),

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

    assert_vars!(instance, 0 => 0, 1 => 0);

    var_add!(instance, 0, 5);
    var_add!(instance, 1, 10);

    assert_vars!(instance, 0 => 5, 1 => 10);
    assert_storage!(instance, 0 => [0, 0, 0, 5], 1 => [0, 10]);
}
