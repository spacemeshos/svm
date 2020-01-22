mod buffer;
mod host_ctx;
mod register;
mod storage;

pub use buffer::{
    buffer_copy_to_reg, buffer_copy_to_storage, buffer_create, buffer_freeze, buffer_kill,
};
pub use host_ctx::host_ctx_read_into_reg;
pub use register::{reg_pop, reg_push};
pub use storage::{
    mem_to_reg_copy, reg_to_mem_copy, storage_read_to_mem, storage_read_to_reg,
    storage_write_from_mem, storage_write_from_reg,
};

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

use svm_storage::{
    page::{PageIndex, PageOffset, PageSliceLayout},
    AppStorage,
};

/// Injects into namespace `ns` the `SVM` internal vmcalls.
pub fn insert_vmcalls(ns: &mut Namespace) {
    // `storage` vmcalls
    ns.insert("mem_to_reg_copy", func!(storage::mem_to_reg_copy));
    ns.insert("reg_to_mem_copy", func!(storage::reg_to_mem_copy));
    ns.insert("storage_read_to_reg", func!(storage::storage_read_to_reg));
    ns.insert("storage_read_to_mem", func!(storage::storage_read_to_mem));
    ns.insert(
        "storage_write_from_mem",
        func!(storage::storage_write_from_mem),
    );
    ns.insert(
        "storage_write_from_reg",
        func!(storage::storage_write_from_reg),
    );

    // `register` vmcalls
    ns.insert("reg_push", func!(reg_push));
    ns.insert("reg_pop", func!(reg_pop));

    // `buffer` vmcalls
    ns.insert("buffer_create", func!(buffer::buffer_create));
    ns.insert("buffer_kill", func!(buffer::buffer_kill));
    ns.insert("buffer_freeze", func!(buffer::buffer_freeze));
    ns.insert(
        "buffer_copy_to_storage",
        func!(buffer::buffer_copy_to_storage),
    );
    ns.insert("buffer_copy_to_reg", func!(buffer::buffer_copy_to_reg));

    // `host_ctx` vmcalls
    ns.insert(
        "host_ctx_read_into_reg",
        func!(host_ctx::host_ctx_read_into_reg),
    );
}
