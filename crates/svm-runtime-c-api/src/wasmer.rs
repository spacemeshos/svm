use std::ffi::c_void;
use std::slice;
use std::sync::Arc;

use crate::{svm_import_func_sig_t, svm_import_func_t, svm_value_type};

use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
};

impl Into<Type> for &svm_value_type {
    fn into(self) -> Type {
        match self {
            svm_value_type::SVM_I32 => Type::I32,
            svm_value_type::SVM_I64 => Type::I64,
        }
    }
}

pub(crate) unsafe fn to_wasmer_import_func(func: *mut svm_import_func_t) -> Export {
    let svm_func: svm_import_func_t = *Box::from_raw(func);

    let func_ptr = svm_func.func as *mut c_void;
    let wasmer_sig = to_wasmer_func_sig(&svm_func.sig);

    Export::Function {
        func: FuncPointer::new(func_ptr as _),
        ctx: Context::Internal,
        signature: Arc::new(wasmer_sig),
    }
}

unsafe fn to_wasmer_func_sig(sig: &svm_import_func_sig_t) -> FuncSig {
    let params = to_wasmer_types_vec(sig.params, sig.params_len);
    let returns = to_wasmer_types_vec(sig.returns, sig.returns_len);

    FuncSig::new(params, returns)
}

unsafe fn to_wasmer_types_vec(types: *const svm_value_type, types_len: u32) -> Vec<Type> {
    let slice = slice::from_raw_parts(types, types_len as usize);
    slice.iter().map(|ty| ty.into()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn svm_value_type_into_wasmer_type() {
        assert_eq!(Type::I32, (&svm_value_type::SVM_I32).into());
        assert_eq!(Type::I64, (&svm_value_type::SVM_I64).into());
    }
}
