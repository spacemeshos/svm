/// The `include_extern_storage` will be imported by Smart Contracts.
#[macro_export]
macro_rules! include_extern_storage_vmcalls {
    () => {
        extern "C" {
            fn mem_to_reg_copy(
                src_mem_idx: i32,
                src_mem_ptr: i32,
                len: i32,
                dst_reg_bits: i32,
                dst_reg_idx: i32,
            );

            fn reg_to_mem_copy(
                src_reg_bits: i32,
                src_reg_idx: i32,
                len: i32,
                dst_mem_idx: i32,
                dst_mem_ptr: i32,
            );

            fn storage_read_to_reg(
                src_page: i32,
                src_slice: i32,
                offset: i32,
                len: i32,
                dst_reg_bits: i32,
                dst_reg_idx: i32,
            );

            fn storage_read_to_mem(
                src_page: i32,
                src_slice: i32,
                offset: i32,
                len: i32,
                dst_mem_idx: i32,
                dst_mem_ptr: i32,
            );

            fn storage_write_from_mem(
                src_mem_idx: i32,
                src_mem_ptr: i32,
                len: i32,
                dst_page: i32,
                dst_slice: i32,
                dst_offset: i32,
            );

            fn storage_write_from_reg(
                src_reg_bits: i32,
                src_reg_idx: i32,
                len: i32,
                dst_page: i32,
                dst_slice: i32,
                dst_offset: i32,
            );
        }
    };
}
