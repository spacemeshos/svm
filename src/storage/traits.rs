/// `RawStorage` - This is the low-level interface for dealing with a contract's storage.
/// For performance concerns, we work on pages units (a page is 4096 bytes)
/// Each read / write operation will involve exactly one page
pub trait RawStorage {
    fn read_page(page: i32);

    fn write_page(page: i32);
}

/// The `declare_storage_api` will be imported by the smart contract wasm programs.
/// Reading data will be copied from the backing storage into temporary buffers.
/// Then the smart contract program will copy that data into the wasm instance memory / stack.
macro_rules! declare_storage_api {
    () => {{
        extern "C" {
            fn mem_copy_to_reg_copy(reg: i32, mem_ptr: i32, mem_len: i32);

            fn reg_copy_to_mem_copy(reg: i32, mem_ptr: i32, mem_len: i32);

            fn storage_read_to_reg(var_page: i32, var_page_offset: i32, var_len: i32, reg: i32);

            fn storage_set_from_reg(var_page: i32, var_page_offset: i32, reg: i32);
        }
    }};
}
