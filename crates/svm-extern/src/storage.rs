/// This macro includes all the storage vmcalls.
#[macro_export]
macro_rules! include_storage_vmcalls {
    () => {
        extern "C" {
            fn mem_to_reg_copy(
                mem_idx: u32,
                mem_offset: u32,
                reg_bits: u32,
                reg_idx: u32,
                count: u32,
            );

            fn reg_to_mem_copy(
                reg_bits: u32,
                reg_idx: u32,
                mem_idx: u32,
                mem_offset: u32,
                count: u32,
            );

            fn storage_read_to_reg(page_offset: u32, reg_bits: u32, reg_idx: u32, count: u32);

            fn storage_read_to_mem(
                page_idx: u32,
                page_offset: u32,
                mem_idx: u32,
                mem_offset: u32,
                count: u32,
            );

            fn storage_write_from_mem(
                mem_idx: u32,
                mem_offset: u32,
                page_idx: u32,
                page_offset: u32,
                count: u32,
            );

            fn storage_write_from_reg(
                reg_bits: u32,
                reg_idx: u32,
                page_idx: u32,
                page_offset: u32,
                count: u32,
            );

            fn storage_read_i32_be(page_idx: u32, page_offset: u32, count: u32) -> u32;

            fn storage_read_i32_le(page_idx: u32, page_offset: u32, count: u32) -> u32;

            fn storage_read_i64_be(page_idx: u32, page_offset: u32, count: u32) -> u64;

            fn storage_read_i64_le(page_idx: u32, page_offset: u32, count: u32) -> u64;
        }
    };
}
