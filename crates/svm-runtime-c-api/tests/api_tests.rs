#![allow(unused)]

use std::ffi::c_void;
use std::sync::Arc;

use wasmer_runtime_c_api::{
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

extern "C" fn func2(ctx: *mut wasmer_instance_context_t, param1: i32, param2: i32) -> i64 {
    0
}

unsafe fn wasmer_import_func_build(
    func: *mut c_void,
    params: Vec<Type>,
    returns: Vec<Type>,
) -> *mut wasmer_import_func_t {
    // let func: *const Func = &Func(func) as _;

    let export = Export::Function {
        func: FuncPointer::new(func as _),
        ctx: Context::Internal,
        signature: Arc::new(FuncSig::new(params, returns)),
    };

    let import: *mut wasmer_import_func_t = Box::into_raw(Box::new(export)) as _;
    import
}

#[test]
fn sanity() {
    // let params = vec![Type::I32, Type::I32];
    // let returns = vec![Type::I64];
}
