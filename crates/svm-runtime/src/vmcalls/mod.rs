mod host_ctx;
mod storage;

pub use host_ctx::host_get64;
pub use storage::{get64, set64};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn wasmer_register(ns: &mut Namespace) {
    // get / set
    ns.insert("get64", func!(get64));
    ns.insert("set64", func!(set64));

    // `host context`
    ns.insert("host_get64", func!(host_get64));
}
