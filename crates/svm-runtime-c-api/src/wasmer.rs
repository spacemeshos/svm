use std::convert::TryFrom;
use std::io;
use std::sync::Arc;

use crate::{svm_byte_array, svm_import_func_sig_t, svm_import_t, svm_import_value};

use svm_app::types::WasmType;

use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
};

pub(crate) unsafe fn to_wasmer_import_func(import: &svm_import_t) -> Export {
    match import.value {
        svm_import_value::Func(ref func) => {
            let wasmer_sig = to_wasmer_func_sig(&func.sig);
            let ptr = func.func.as_ptr();

            Export::Function {
                func: FuncPointer::new(ptr as _),
                ctx: Context::Internal,
                signature: Arc::new(wasmer_sig),
            }
        }
    }
}

unsafe fn to_wasmer_func_sig(sig: &svm_import_func_sig_t) -> FuncSig {
    let params = to_wasmer_types_vec(sig.params);
    let returns = to_wasmer_types_vec(sig.returns);

    FuncSig::new(params, returns)
}

#[inline]
unsafe fn to_wasmer_types_vec(types: svm_byte_array) -> Vec<Type> {
    let types: Result<Vec<WasmType>, io::Error> = Vec::try_from(types);

    let types = types.unwrap();

    types
        .iter()
        .map(|ty| match ty {
            WasmType::I32 => Type::I32,
            WasmType::I64 => Type::I64,
        })
        .collect()
}
