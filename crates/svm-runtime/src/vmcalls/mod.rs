mod buffer;
mod host_ctx;
mod register;
mod storage;

pub use buffer::{buffer_copy_to_reg, buffer_create, buffer_freeze, buffer_kill};
pub use host_ctx::{
    host_ctx_read_i32_be, host_ctx_read_i32_le, host_ctx_read_i64_be, host_ctx_read_i64_le,
    host_ctx_read_into_reg, host_get64,
};
pub use register::{
    reg_cmp, reg_pop, reg_push, reg_set_i32_be, reg_set_i32_le, reg_set_i64_be, reg_set_i64_le,
};
pub use storage::{get64, set64};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn insert_vmcalls(ns: &mut Namespace) {
    // `register` vmcalls
    ns.insert("reg_push", func!(reg_push));
    ns.insert("reg_pop", func!(reg_pop));
    ns.insert("reg_cmp", func!(reg_cmp));
    ns.insert("reg_set_i32_be", func!(reg_set_i32_be));
    ns.insert("reg_set_i32_le", func!(reg_set_i32_le));
    ns.insert("reg_set_i64_be", func!(reg_set_i64_be));
    ns.insert("reg_set_i64_le", func!(reg_set_i64_le));

    // `buffer` vmcalls
    ns.insert("buffer_create", func!(buffer_create));
    ns.insert("buffer_kill", func!(buffer_kill));
    ns.insert("buffer_freeze", func!(buffer_freeze));
    ns.insert("buffer_copy_to_reg", func!(buffer_copy_to_reg));

    // `host_ctx` vmcalls
    ns.insert("host_ctx_read_into_reg", func!(host_ctx_read_into_reg));
    ns.insert("host_ctx_read_i32_be", func!(host_ctx_read_i32_be));
    ns.insert("host_ctx_read_i32_le", func!(host_ctx_read_i32_le));
    ns.insert("host_ctx_read_i64_be", func!(host_ctx_read_i64_be));
    ns.insert("host_ctx_read_i64_le", func!(host_ctx_read_i64_le));

    // for v0.2
    ns.insert("get64", func!(get64));
    ns.insert("set64", func!(set64));
    ns.insert("host_get64", func!(host_get64));
}
