mod buffer;
mod host_ctx;
mod storage;

pub use buffer::{buffer_create, buffer_freeze, buffer_kill};
pub use host_ctx::{
    host_ctx_read_i32_be, host_ctx_read_i32_le, host_ctx_read_i64_be, host_ctx_read_i64_le,
    host_get64,
};
pub use storage::{get160, get256, get64, set160, set256, set64};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn insert_vmcalls(ns: &mut Namespace) {
    // get / set
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

    // `host_ctx` vmcalls
    ns.insert("host_ctx_read_i32_be", func!(host_ctx_read_i32_be));
    ns.insert("host_ctx_read_i32_le", func!(host_ctx_read_i32_le));
    ns.insert("host_ctx_read_i64_be", func!(host_ctx_read_i64_be));
    ns.insert("host_ctx_read_i64_le", func!(host_ctx_read_i64_le));
}
