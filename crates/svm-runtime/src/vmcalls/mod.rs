mod register;
mod storage;

/// When called, injects the code of the `svm vmcalls`.
/// The `vmcalls` are functions imported into each running `svm` instance.
#[macro_export]
macro_rules! include_svm_vmcalls {
    ($PC: path) => {
        mod vmcalls {
            $crate::include_svm_storage_vmcalls!($PC);
            $crate::include_svm_register_vmcalls!($PC);
        }
    };
}
