use wasmer::{Exports, Function, ImportObject, Store};

use crate::Context;

mod calldata;
mod host_ctx;
mod logs;
mod storage;

pub use calldata::{calldata_len, calldata_ptr};
pub use host_ctx::host_get64;
pub use logs::log;
pub use storage::{get32, get64, load160, load256, set32, set64, store160, store256};

macro_rules! func {
    ($store:ident, $ctx:ident, $f:expr) => {{
        // Each host function own it's own `Context`
        let ctx = $ctx.clone();

        Function::new_native_with_env($store, ctx, $f)
    }};
}

pub fn wasmer_register(store: &Store, ctx: &Context, ns: &mut Exports) {
    ns.insert("calldata_ptr", func!(store, ctx, calldata_ptr));
    ns.insert("calldata_len", func!(store, ctx, calldata_len));

    ns.insert("host_get64", func!(store, ctx, host_get64));

    ns.insert("get32", func!(store, ctx, get32));
    ns.insert("set32", func!(store, ctx, set32));

    ns.insert("get64", func!(store, ctx, get64));
    ns.insert("set64", func!(store, ctx, set64));

    ns.insert("load160", func!(store, ctx, load160));
    ns.insert("store160", func!(store, ctx, store160));

    ns.insert("load256", func!(store, ctx, load256));
    ns.insert("store256", func!(store, ctx, store256));

    ns.insert("log", func!(store, ctx, log));
}
