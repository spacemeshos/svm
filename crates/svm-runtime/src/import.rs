use std::ffi::c_void;
use std::ptr;

use crate::Context;
use svm_types::WasmType;

use wasmer::{Export, FunctionType, Type as WasmerType};
use wasmer_vm::{ExportFunction, VMContext, VMFunctionKind};

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub name: String,

    pub params: Vec<WasmType>,

    pub returns: Vec<WasmType>,

    pub func_ptr: *const c_void,
}

impl Import {
    pub fn to_wasmer(&self, ctx: Context) -> Export {
        let params = to_wasmer_types(&self.params);
        let returns = to_wasmer_types(&self.returns);
        let signature = FunctionType::new(params, returns);

        // TODO: needs to free `ctx`.
        let boxed_ctx = Box::new(ctx);
        let vmctx = Box::into_raw(boxed_ctx) as *mut _ as *mut VMContext;

        let func = ExportFunction {
            address: self.func_ptr as _,
            vmctx,
            signature,
            kind: VMFunctionKind::Static,
            call_trampoline: None,
        };

        func.into()
    }
}

fn to_wasmer_types(types: &[WasmType]) -> Vec<WasmerType> {
    types
        .iter()
        .map(|ty| match ty {
            WasmType::I32 => WasmerType::I32,
            WasmType::I64 => WasmerType::I64,
            _ => panic!("Only i32 and i64 are supported."),
        })
        .collect()
}
