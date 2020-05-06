mod buffer;
mod host_ctx;
mod register;
mod storage;
mod storage2;

pub use buffer::{
    buffer_copy_to_reg, buffer_copy_to_storage, buffer_create, buffer_freeze, buffer_kill,
};
pub use host_ctx::{
    host_ctx_read_i32_be, host_ctx_read_i32_le, host_ctx_read_i64_be, host_ctx_read_i64_le,
    host_ctx_read_into_reg,
};
pub use register::{
    reg_cmp, reg_pop, reg_push, reg_set_i32_be, reg_set_i32_le, reg_set_i64_be, reg_set_i64_le,
};
pub use storage::{
    mem_to_reg_copy, reg_to_mem_copy, storage_read_i32_be, storage_read_i32_le,
    storage_read_i64_be, storage_read_i64_le, storage_read_to_mem, storage_read_to_reg,
    storage_write_from_mem, storage_write_from_reg, storage_write_i32_be, storage_write_i32_le,
    storage_write_i64_be, storage_write_i64_le,
};
pub use storage2::{get64, set64};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn insert_vmcalls(ns: &mut Namespace) {
    // `storage` vmcalls
    ns.insert("mem_to_reg_copy", func!(mem_to_reg_copy));
    ns.insert("reg_to_mem_copy", func!(reg_to_mem_copy));
    ns.insert("storage_read_to_reg", func!(storage_read_to_reg));
    ns.insert("storage_read_to_mem", func!(storage_read_to_mem));
    ns.insert("storage_write_from_mem", func!(storage_write_from_mem));
    ns.insert("storage_write_from_reg", func!(storage_write_from_reg));

    ns.insert("storage_read_i32_be", func!(storage_read_i32_be));
    ns.insert("storage_read_i32_le", func!(storage_read_i32_le));
    ns.insert("storage_read_i64_be", func!(storage_read_i64_be));
    ns.insert("storage_read_i64_le", func!(storage_read_i64_le));

    ns.insert("storage_write_i32_be", func!(storage_write_i32_be));
    ns.insert("storage_write_i32_le", func!(storage_write_i32_le));
    ns.insert("storage_write_i64_be", func!(storage_write_i64_be));
    ns.insert("storage_write_i64_le", func!(storage_write_i64_le));

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
    ns.insert("buffer_copy_to_storage", func!(buffer_copy_to_storage));
    ns.insert("buffer_copy_to_reg", func!(buffer::buffer_copy_to_reg));

    // `host_ctx` vmcalls
    ns.insert("host_ctx_read_into_reg", func!(host_ctx_read_into_reg));
    ns.insert("host_ctx_read_i32_be", func!(host_ctx_read_i32_be));
    ns.insert("host_ctx_read_i32_le", func!(host_ctx_read_i32_le));
    ns.insert("host_ctx_read_i64_be", func!(host_ctx_read_i64_be));
    ns.insert("host_ctx_read_i64_le", func!(host_ctx_read_i64_le));

    // for v0.2
    ns.insert("get64", func!(get64));
    ns.insert("set64", func!(set64));
}
