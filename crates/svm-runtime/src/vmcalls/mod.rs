mod host_ctx;
mod storage;

pub use host_ctx::host_get64;
pub use storage::{get32, get64, set32, set64};

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

    ns.insert("host_get64", func!(host_get64));
}
