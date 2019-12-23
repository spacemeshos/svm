/// The `include_extern_storage` will be imported by Smart Contracts.
#[macro_export]
macro_rules! include_extern_storage_vmcalls {
    () => {
        extern "C" {
            fn mem_to_reg_copy(
                mem_idx: i32,
                mem_offset: i32,
                len: i32,
                reg_bits: i32,
                reg_idx: i32,
            );

            fn reg_to_mem_copy(
                reg_bits: i32,
                reg_idx: i32,
                len: i32,
                mem_idx: i32,
                mem_offset: i32,
            );

            fn storage_read_to_reg(page: i32, offset: i32, len: i32, reg_bits: i32, reg_idx: i32);

            fn storage_read_to_mem(page: i32, offset: i32, len: i32, mem_idx: i32, mem_offset: i32);

            fn storage_write_from_mem(
                mem_idx: i32,
                mem_offset: i32,
                len: i32,
                page_idx: i32,
                page_offset: i32,
            );

            fn storage_write_from_reg(
                reg_bits: i32,
                reg_idx: i32,
                len: i32,
                page_idx: i32,
                page_offset: i32,
            );
        }
    };
}
