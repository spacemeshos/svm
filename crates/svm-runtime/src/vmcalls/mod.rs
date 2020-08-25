use wasmer::{Exports, Function, ImportObject, Store};

use crate::ctx::SvmCtx;

mod calldata;
mod host_ctx;
mod logs;
mod storage;

pub use calldata::{calldata_len, calldata_ptr};
pub use host_ctx::host_get64;
pub use logs::log;
pub use storage::{get32, get64, load160, load256, set32, set64, store160, store256};

macro_rules! func {
    ($store:ident, $env:ident, $f:expr) => {{
        Function::new_native_with_env($store, $env.clone(), $f)
    }};
}

pub fn wasmer_register(store: &Store, env: SvmCtx, ns: &mut Exports) {
    ns.insert("calldata_ptr", func!(store, env, calldata_ptr));
    ns.insert("calldata_len", func!(store, env, calldata_len));

    ns.insert("host_get64", func!(store, env, host_get64));

    ns.insert("get32", func!(store, env, get32));
    ns.insert("set32", func!(store, env, set32));

    ns.insert("get64", func!(store, env, get64));
    ns.insert("set64", func!(store, env, set64));

    ns.insert("load160", func!(store, env, load160));
    ns.insert("store160", func!(store, env, store160));

    ns.insert("load256", func!(store, env, load256));
    ns.insert("store256", func!(store, env, store256));

    ns.insert("log", func!(store, env, log));
}
