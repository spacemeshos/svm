use std::sync::Arc;

use crate::import::{Import, ImportFuncSig, ImportValue};

use svm_app::types::WasmType;

use wasmer_runtime_core::{
    export::{Context as WasmerCtx, Export as WasmerExport, FuncPointer as WasmerFuncPtr},
    types::{FuncSig as WasmerFuncSig, Type as WasmerType},
};

pub(crate) unsafe fn to_wasmer_import_func(import: &Import) -> WasmerExport {
    match import.value {
        ImportValue::Func(ref func) => {
            let wasmer_sig = to_wasmer_func_sig(&func.sig);
            let ptr = func.func.as_ptr();

            WasmerExport::Function {
                func: WasmerFuncPtr::new(ptr as _),
                ctx: WasmerCtx::Internal,
                signature: Arc::new(wasmer_sig),
            }
        }
    }
}

unsafe fn to_wasmer_func_sig(sig: &ImportFuncSig) -> WasmerFuncSig {
    let params = to_wasmer_types_vec(&sig.params);
    let returns = to_wasmer_types_vec(&sig.returns);

    WasmerFuncSig::new(params, returns)
}

#[inline]
unsafe fn to_wasmer_types_vec(types: &[WasmType]) -> Vec<WasmerType> {
    types
        .iter()
        .map(|ty| match ty {
            WasmType::I32 => WasmerType::I32,
            WasmType::I64 => WasmerType::I64,
        })
        .collect()
}
