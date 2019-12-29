use crate::svm_value_type;
use wasmer_runtime_core::types::Type;

#[allow(non_snake_case)]
impl Into<Type> for &svm_value_type {
    fn into(self) -> Type {
        match self {
            I32 => Type::I32,
            I64 => Type::I64,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn svm_value_type_into_wasmer_type() {
        assert_eq!(Type::I32, (&svm_value_type::I32).into());
        assert_ne!(Type::I64, (&svm_value_type::I64).into());
    }
}
