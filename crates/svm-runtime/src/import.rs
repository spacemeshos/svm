use std::ffi::c_void;
use std::ptr;

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
    pub fn to_wasmer<E>(&self, env: E) -> Export {
        let params = to_wasmer_types(&self.params);
        let returns = to_wasmer_types(&self.returns);
        let signature = FunctionType::new(params, returns);

        let boxed_env = Box::new(env);
        let vmctx = Box::into_raw(boxed_env) as *mut VMContext;

        let func = ExportFunction {
            address: self.func_ptr as _,
            vmctx,
            signature,
            kind: VMFunctionKind::Static,
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
