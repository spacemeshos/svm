use std::sync::Arc;

use crate::{svm_import_func_sig_t, svm_import_t, svm_import_value, svm_value_type};

use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
};

impl From<&svm_value_type> for Type {
    fn from(value: &svm_value_type) -> Type {
        if value.SVM_I32_ON == value.SVM_I64_ON {
            unreachable!()
        }

        if value.SVM_I32_ON {
            Type::I32
        } else {
            Type::I64
        }
    }
}

impl From<svm_value_type> for Type {
    fn from(value: svm_value_type) -> Type {
        (&value).into()
    }
}

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
    let params = to_wasmer_types_vec(&sig.params[..]);
    let returns = to_wasmer_types_vec(&sig.returns[..]);

    FuncSig::new(params, returns)
}

#[inline]
unsafe fn to_wasmer_types_vec(types: &[svm_value_type]) -> Vec<Type> {
    types.iter().map(|ty| ty.into()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn svm_value_type_into_wasmer_type() {
        assert_eq!(
            Type::I32,
            svm_value_type {
                SVM_I32_ON: true,
                SVM_I64_ON: false,
            }
            .into()
        );
        assert_eq!(
            Type::I64,
            svm_value_type {
                SVM_I32_ON: false,
                SVM_I64_ON: true
            }
            .into()
        );

        assert_eq!(
            Type::I32,
            (&svm_value_type {
                SVM_I32_ON: true,
                SVM_I64_ON: false
            })
                .into()
        );

        assert_eq!(
            Type::I64,
            (&svm_value_type {
                SVM_I32_ON: false,
                SVM_I64_ON: true
            })
                .into()
        );
    }
}
