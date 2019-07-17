/// A utility macro for computing hashes using `DefaultPageIndexHasher`
#[macro_export]
macro_rules! default_page_hash {
    ($addr: expr, $page_idx: expr) => {{
        use crate::default::DefaultPageIndexHasher;
        use crate::page::PageIndex;
        use crate::traits::PageIndexHasher;

        let addr = Address::from($addr as u32);

        DefaultPageIndexHasher::hash(addr, PageIndex($page_idx))
    }};
}

/// The `include_extern_storage` will be imported by Smart Contracts wasm programs.
/// Reading data will be copied from the backing storage into temporary buffers.
/// Then the smart contract program will copy that data into the wasm instance's memory / stack.
#[macro_export]
macro_rules! include_extern_storage {
    () => {
        extern "C" {
            fn mem_to_reg_copy(src_mem_ptr: i32, len: i32, dst_reg: i32);

            fn reg_to_mem_copy(src_reg: i32, len: i32, dst_mem_ptr: i32);

            fn reg_to_mem_copy(src_reg: i32, len: i32, dst_mem_ptr: i32);

            fn storage_read_to_reg(
                src_page: i32,
                src_slice: i32,
                offset: i32,
                len: i32,
                dst_reg: i32,
            );

            fn storage_read_to_mem(
                src_page: i32,
                src_slice: i32,
                offset: i32,
                len: i32,
                dst_mem_ptr: i32,
            );

            fn storage_write_from_mem(
                src_mem_ptr: i32,
                len: i32,
                dst_page: i32,
                dst_slice: i32,
                dst_offset: i32,
            );
        }
    };
}
