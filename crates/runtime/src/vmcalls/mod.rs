//! Implements the `SVM` vmcalls (a.k.a `libcalls / hostcalls / syscalls`)

use wasmer::{Exports, Function, Store};

use crate::FuncEnv;

mod alloc;
mod calldata;
mod logs;
mod returndata;
mod storage;

pub use alloc::static_alloc;
pub use calldata::{calldata_len, calldata_offset};
pub use logs::log;
pub use returndata::set_returndata;
pub use storage::{get32, get64, load160, set32, set64, store160};

macro_rules! func {
    ($store:ident, $env:ident, $f:expr) => {{
        // Each host function owns its own `Context`.
        let env = $env.clone();

        Function::new_native_with_env($store, env, $f)
    }};
}

/// Registers SVM internal host functions (a.k.a `vmcalls`)
/// into `Wasmer` Import Object (it's done by inserting to input `Exports`)
pub fn wasmer_register(store: &Store, env: &FuncEnv, ns: &mut Exports) {
    ns.insert("svm_static_alloc", func!(store, env, static_alloc));

    ns.insert("svm_calldata_offset", func!(store, env, calldata_offset));
    ns.insert("svm_calldata_len", func!(store, env, calldata_len));
    ns.insert("svm_set_returndata", func!(store, env, set_returndata));

    ns.insert("svm_get32", func!(store, env, get32));
    ns.insert("svm_set32", func!(store, env, set32));

    ns.insert("svm_get64", func!(store, env, get64));
    ns.insert("svm_set64", func!(store, env, set64));

    ns.insert("svm_load160", func!(store, env, load160));
    ns.insert("svm_store160", func!(store, env, store160));

    ns.insert("svm_log", func!(store, env, log));
}
