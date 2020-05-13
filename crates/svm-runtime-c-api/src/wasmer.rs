use std::sync::Arc;

use crate::{svm_import_func_sig_t, svm_import_t, svm_import_value};
use svm_app::types::WasmType;

use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
};

pub(crate) unsafe fn to_wasmer_import_func(import: &svm_import_t) -> Export {
    todo!()
    // match import.value {
    //     svm_import_value::Func(ref func) => {
    //         let wasmer_sig = to_wasmer_func_sig(&func.sig);
    //         let ptr = func.func.as_ptr();

    //         Export::Function {
    //             func: FuncPointer::new(ptr as _),
    //             ctx: Context::Internal,
    //             signature: Arc::new(wasmer_sig),
    //         }
    //     }
    // }
}

unsafe fn to_wasmer_func_sig(sig: &svm_import_func_sig_t) -> FuncSig {
    todo!()
    // let params = to_wasmer_types_vec(&sig.params[..]);
    // let returns = to_wasmer_types_vec(&sig.returns[..]);

    // FuncSig::new(params, returns)
}

#[inline]
unsafe fn to_wasmer_types_vec(types: &[WasmType]) -> Vec<Type> {
    todo!()
    // types.iter().map(|ty| ty.into()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn svm_value_type_into_wasmer_type() {
        assert_eq!(Type::I32, svm_value_type::SVM_I32.into());
        assert_eq!(Type::I64, svm_value_type::SVM_I64.into());

        assert_eq!(Type::I32, (&svm_value_type::SVM_I32).into());
        assert_eq!(Type::I64, (&svm_value_type::SVM_I64).into());
    }
}
