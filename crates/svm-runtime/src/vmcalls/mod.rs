use wasmer::{Exports, Function, ImportObject, Store};

use crate::Context;

mod calldata;
mod host_ctx;
mod logs;
mod returndata;
mod storage;

pub use calldata::{calldata_len, calldata_offset};
pub use host_ctx::host_get64;
pub use logs::log;
pub use returndata::set_returndata;
pub use storage::{get32, get64, load160, set32, set64, store160};

macro_rules! func {
    ($store:ident, $ctx:ident, $f:expr) => {{
        // Each host function own it's own `Context`
        let ctx = $ctx.clone();

        Function::new_native_with_env($store, ctx, $f)
    }};
}

pub fn wasmer_register(store: &Store, ctx: &Context, ns: &mut Exports) {
    ns.insert("calldata_offset", func!(store, ctx, calldata_offset));
    ns.insert("calldata_len", func!(store, ctx, calldata_len));
    ns.insert("set_returndata", func!(store, ctx, set_returndata));

    ns.insert("host_get64", func!(store, ctx, host_get64));

    ns.insert("get32", func!(store, ctx, get32));
    ns.insert("set32", func!(store, ctx, set32));

    ns.insert("get64", func!(store, ctx, get64));
    ns.insert("set64", func!(store, ctx, set64));

    ns.insert("load160", func!(store, ctx, load160));
    ns.insert("store160", func!(store, ctx, store160));

    ns.insert("log", func!(store, ctx, log));
}
