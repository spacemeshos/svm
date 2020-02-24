/// This macro includes all the buffer vmcalls.
#[macro_export]
macro_rules! include_buffer_vmcalls {
    () => {
        extern "C" {
            fn buffer_create(buf_id: u32, capacity: u32);

            fn buffer_kill(buf_id: u32);

            fn buffer_freeze(buf_id: u32);

            fn buffer_copy_to_storage(
                buf_id: u32,
                buf_offset: u32,
                page_idx: u32,
                page_offset: u32,
                count: u32,
            );

            fn buffer_copy_to_reg(
                buf_id: u32,
                buf_offset: u32,
                reg_bits: u32,
                reg_idx: u32,
                count: u32,
            );
        }
    };
}
