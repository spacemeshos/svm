use std::ffi::c_void;

use wasmer_runtime::{func, imports, Func};

use svm_app::types::HostCtx;
use svm_common::{Address, State};
use svm_runtime::{
    gas::MaybeGas,
    helpers::{self, DataWrapper},
    testing::{self, instance_buffer, instance_register, instance_storage},
    vmcalls,
};
use svm_storage2::app::AppStorage as AppStorage2;

fn default_test_args() -> (
    Address,
    State,
    DataWrapper<*mut c_void>,
    DataWrapper<*const c_void>,
    MaybeGas,
    u16,
) {
    let app_addr = Address::of("my-app");
    let state = State::empty();
    let host = DataWrapper::new(std::ptr::null_mut());
    let host_ctx = DataWrapper::new(svm_common::into_raw(HostCtx::new()));
    let maybe_gas = MaybeGas::new();

    (app_addr, state, host, host_ctx, maybe_gas)
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
    todo!()
}
