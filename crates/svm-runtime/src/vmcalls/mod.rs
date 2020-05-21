mod buffer;
mod host_ctx;
mod storage;

pub use buffer::{buffer_create, buffer_freeze, buffer_kill};
pub use host_ctx::host_get64;
pub use storage::{get64, set64};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn insert_vmcalls(ns: &mut Namespace) {
    // `buffer` vmcalls
    ns.insert("buffer_create", func!(buffer_create));
    ns.insert("buffer_kill", func!(buffer_kill));
    ns.insert("buffer_freeze", func!(buffer_freeze));

    // get / set
    ns.insert("get64", func!(get64));
    ns.insert("set64", func!(set64));

    // `host context`
    ns.insert("host_get64", func!(host_get64));
}
