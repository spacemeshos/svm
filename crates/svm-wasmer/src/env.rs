use wasmer_runtime::{func, imports, ImportObject};
use wasmer_runtime_core::import::Namespace;

use crate::include_wasmer_svm_vmcalls;

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

    use svm_storage::{
        default::DefaultPageCache,
        memory::{MemKVStore, MemPages},
    };

    pub type MemPageCache<'pc, K = [u8; 32]> = DefaultPageCache<'pc, MemPages<K>>;

    include_wasmer_svm_env!(MemPageCache);

    #[test]
    fn svm_import_object() {
        let import_object = svm_import_object!();
    }
}
