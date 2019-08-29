mod register;
mod storage;

/// When called, injects the code of the `svm wasmer vmcalls`.
/// The `vmcalls` are functions imported into each running `svm wasmer` instance.
#[macro_export]
macro_rules! include_wasmer_svm_vmcalls {
    ($PC: path) => {
        $crate::include_wasmer_svm_storage_vmcalls!($PC);
        $crate::include_wasmer_svm_register_vmcalls!($PC);
    };
}
