use wasmer_runtime::{func, imports, ImportObject};
use wasmer_runtime_core::import::Namespace;

use crate::include_wasmer_svm_vmcalls;

/// When called, injects the code of the `svm wasmer vmcalls` and the code that builds the initial import object
/// containing the vmcalls.
#[macro_export]
macro_rules! include_wasmer_svm_env {
    ($PC: ident) => {
        include_wasmer_svm_vmcalls!($PC);

        macro_rules! svm_import_object {
            () => {
                imports! {
                    "env" => {
                        "mem_to_reg_copy" => func!(mem_to_reg_copy),
                        // ....
                        // ....
                    },
                }
            };
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_storage::memory::MemPageCache32;

    include_wasmer_svm_env!(MemPageCache32);

    #[test]
    fn svm_import_object() {
        let import_object = svm_import_object!();
    }
}
