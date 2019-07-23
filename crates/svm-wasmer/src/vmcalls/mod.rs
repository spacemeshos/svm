mod register_arithmetic;
mod storage;

#[macro_export]
macro_rules! include_wasmer_svm_vmcalls {
    ($PC: ident) => {
        crate::include_wasmer_svm_storage_vmcalls!($PC);
        crate::include_wasmer_svm_register_vmcalls!($PC);
    };
}
