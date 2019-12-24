#![allow(unused)]

extern crate svm_runtime_c_api;

use svm_runtime_c_api as api;

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Arc;

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

unsafe fn wasmer_import_func_build(
    func: *mut c_void,
    params: Vec<Type>,
    returns: Vec<Type>,
) -> *mut wasmer_import_func_t {
    let func: *const Func = &Func(func) as _;

    let export = Export::Function {
        func: FuncPointer::new(func),
        ctx: Context::Internal,
        signature: Arc::new(FuncSig::new(params, returns)),
    };

    Box::into_raw(Box::new(export)) as _
}

unsafe fn wasmer_import_func_destroy(func: *mut wasmer_import_func_t) {
    Box::from_raw(func);
}

fn str_to_wasmer_byte_array(s: &str) -> wasmer_byte_array {
    let (bytes, bytes_len) = str_to_bytes(s);

    wasmer_byte_array { bytes, bytes_len }
}

fn str_to_bytes(s: &str) -> (*const u8, u32) {
    let bytes = s.as_ptr();
    let bytes_len = s.len() as u32;

    (bytes, bytes_len)
}

fn wasmer_import_as_value(func: *const wasmer_import_func_t) -> wasmer_import_export_value {
    wasmer_import_export_value { func }
}

unsafe fn wasmer_import_func_create(
    module_name: &str,
    import_name: &str,
    func: *mut c_void,
    params: Vec<Type>,
    returns: Vec<Type>,
) -> wasmer_import_t {
    let module_name = str_to_wasmer_byte_array(module_name);
    let import_name = str_to_wasmer_byte_array(import_name);
    let func = wasmer_import_func_build(func, params, returns);

    wasmer_import_t {
        module_name,
        import_name,
        tag: wasmer_import_export_kind::WASM_FUNCTION,
        value: wasmer_import_as_value(func),
    }
}

unsafe fn alloc_ptr() -> *mut c_void {
    let ptr: *mut c_void = std::ptr::null_mut();
    let ptr = Box::new(ptr);

    *Box::into_raw(ptr)
}

#[test]
fn sanity() {
    unsafe {
        unsafe_sanity();
    }
}

unsafe fn unsafe_sanity() {
    let get_balance_import = wasmer_import_func_create(
        "env",
        "get_balance",
        &mut vmcall_get_balance as *mut _ as _,
        vec![Type::I32, Type::I32],
        vec![Type::I64],
    );

    let mut host = Host::new();
    let host_ptr: *mut c_void = &mut host as *mut Host as _;

    let mut runtime = alloc_ptr();
    let (path_bytes, path_len) = str_to_bytes("tests");

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
