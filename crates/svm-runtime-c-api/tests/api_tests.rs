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

    fn as_mut_ptr(&mut self) -> *mut c_void {
        self as *mut Self as _
    }
}

extern "C" fn vmcall_get_balance(
    ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
) -> i64 {
    0
}

extern "C" fn vmcall_set_balance(
    ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
    balance: i64,
) {
    //
}

macro_rules! func {
    ($func:ident) => {{
        &mut $func as *mut _ as _
    }};
}

#[test]
fn sanity() {
    unsafe {
        unsafe_sanity();
    }
}

unsafe fn create_imports() -> (Vec<wasmer_import_t>, u32) {
    let get_balance_import = testing::wasmer_import_func_create(
        "env",
        "get_balance",
        func!(vmcall_get_balance),
        vec![Type::I32, Type::I32],
        vec![Type::I64],
    );

    let set_balance_import = testing::wasmer_import_func_create(
        "env",
        "set_balance",
        func!(vmcall_set_balance),
        vec![Type::I32, Type::I32, Type::I64],
        vec![],
    );

    let imports = vec![get_balance_import, set_balance_import];
    let imports_len = imports.len() as u32;

    (imports, imports_len)
}

unsafe fn unsafe_sanity() {
    let mut host = Host::new();
    let mut runtime = std::ptr::null_mut();
    let (mut imports, imports_len) = create_imports();

    let (path_bytes, path_len) = testing::str_to_bytes("tests");

    let res = api::svm_runtime_create(
        &mut runtime,
        path_bytes,
        path_len,
        host.as_mut_ptr(),
        imports.as_mut_ptr() as _,
        imports_len,
    );
    // TODO: assert that `res` is `wasmer_result_t::WASMER_OK`
}
