mod register;
mod storage;

pub use wasmer_runtime_core::{
    func,
    import::{IsExport, Namespace},
};

pub fn insert_vmcalls(ns: &mut Namespace) {
    // storage vmcalls
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

    // register vmcalls
    ("reg_replace_byte", func!(register::reg_replace_byte));
    ("reg_read_be_i64", func!(register::reg_read_be_i64));
    ("reg_write_be_i64", func!(register::reg_write_be_i64));
}
