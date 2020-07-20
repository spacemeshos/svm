mod host_ctx;
mod logs;
mod storage;

pub use host_ctx::host_get64;
pub use logs::log;
pub use storage::{get32, get64, load160, load256, set32, set64, store160, store256};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn wasmer_register(ns: &mut Namespace) {
    ns.insert("get32", func!(get32));
    ns.insert("set32", func!(set32));

    ns.insert("get64", func!(get64));
    ns.insert("set64", func!(set64));

    ns.insert("load160", func!(load160));
    ns.insert("store160", func!(store160));

    ns.insert("load256", func!(load256));
    ns.insert("store256", func!(store256));

    ns.insert("host_get64", func!(host_get64));

    ns.insert("log", func!(log));
}
