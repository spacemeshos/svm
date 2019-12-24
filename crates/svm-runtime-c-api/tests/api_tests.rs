#![allow(unused)]

extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;
use svm_runtime_c_api::testing;

use std::collections::HashMap;
use std::ffi::c_void;

use wasmer_runtime_c_api::{
    export::{wasmer_import_export_kind, wasmer_import_export_value},
    import::{wasmer_import_func_new, wasmer_import_func_t, wasmer_import_t},
    instance::wasmer_instance_context_t,
    value::wasmer_value_tag,
    wasmer_byte_array, wasmer_result_t,
};

use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
    vm::{Ctx, Func},
};

use svm_common::Address;

struct Host {
    balance: HashMap<Address, i64>,
}

impl Host {
    fn new() -> Self {
        Self {
            balance: HashMap::new(),
        }
    }

    fn get_balance(&self, addr: &Address) -> Option<i64> {
        self.balance.get(addr).copied()
    }

    fn set_balance(&mut self, addr: &Address, balance: i64) {
        self.balance.insert(addr.clone(), balance);
    }
}

extern "C" fn vmcall_get_balance(
    ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
) -> i64 {
    0
}

#[test]
fn sanity() {
    unsafe {
        unsafe_sanity();
    }
}

unsafe fn unsafe_sanity() {
    let get_balance_import = testing::wasmer_import_func_create(
        "env",
        "get_balance",
        &mut vmcall_get_balance as *mut _ as _,
        vec![Type::I32, Type::I32],
        vec![Type::I64],
    );

    let mut host = Host::new();
    let host_ptr: *mut c_void = &mut host as *mut Host as _;

    let mut runtime = testing::alloc_ptr();
    let (path_bytes, path_len) = testing::str_to_bytes("tests");

    let imports = std::ptr::null_mut();
    let imports_len = 0;

    let res = api::svm_runtime_create(
        &mut runtime,
        path_bytes,
        path_len,
        host_ptr,
        imports,
        imports_len,
    );

    // TODO: assert that `res` is `wasmer_result_t::WASMER_OK`
}
