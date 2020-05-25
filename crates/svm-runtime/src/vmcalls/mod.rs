mod buffer;
mod host_ctx;
mod storage;

pub use buffer::{buffer_create, buffer_freeze, buffer_kill};
pub use host_ctx::host_get64;
pub use storage::{get160, get256, get32, get64, set160, set256, set32, set64};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn insert_vmcalls(ns: &mut Namespace) {
    // get / set
    ns.insert("get32", func!(get32));
    ns.insert("set32", func!(set32));
    ns.insert("get64", func!(get64));
    ns.insert("set64", func!(set64));
    ns.insert("get160", func!(get160));
    ns.insert("set160", func!(set160));
    ns.insert("get256", func!(get256));
    ns.insert("set256", func!(set256));
    ns.insert("host_get64", func!(host_get64));

    // `buffer` vmcalls
    ns.insert("buffer_create", func!(buffer_create));
    ns.insert("buffer_kill", func!(buffer_kill));
    ns.insert("buffer_freeze", func!(buffer_freeze));

}
