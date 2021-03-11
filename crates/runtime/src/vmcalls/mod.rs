use wasmer::{Exports, Function, Store};

use crate::Context;

mod calldata;
mod logs;
mod returndata;
mod storage;

pub use calldata::{calldata_len, calldata_offset};
pub use logs::log;
pub use returndata::set_returndata;
pub use storage::{get32, get64, load160, set32, set64, store160};

macro_rules! func {
    ($store:ident, $ctx:ident, $f:expr) => {{
        // Each host function owns its own `Context`.
        let ctx = $ctx.clone();

        Function::new_native_with_env($store, ctx, $f)
    }};
}

/// Registers SVM internal host functions (a.k.a vmacalls) into
/// Into `Wasmer` Import Object (it's done by inserting to input `Exports`)
pub fn wasmer_register(store: &Store, ctx: &Context, ns: &mut Exports) {
    ns.insert("svm_calldata_offset", func!(store, ctx, calldata_offset));
    ns.insert("svm_calldata_len", func!(store, ctx, calldata_len));
    ns.insert("svm_set_returndata", func!(store, ctx, set_returndata));

    ns.insert("svm_get32", func!(store, ctx, get32));
    ns.insert("svm_set32", func!(store, ctx, set32));

    ns.insert("svm_get64", func!(store, ctx, get64));
    ns.insert("svm_set64", func!(store, ctx, set64));

    ns.insert("svm_load160", func!(store, ctx, load160));
    ns.insert("svm_store160", func!(store, ctx, store160));

    ns.insert("svm_log", func!(store, ctx, log));
}
