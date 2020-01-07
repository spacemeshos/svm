#![no_std]

svm_extern::include_extern_storage_vmcalls!();
svm_extern::include_extern_node_vmcalls!();
svm_extern::include_extern_register_vmcalls!();

#[no_mangle]
pub extern "C" fn execute(
    src_addr_mem_idx: i32,
    src_addr_mem_ptr: i32,
    dst_addr_mem_idx: i32,
    dst_addr_mem_ptr: i32,
    amount: i64,
) -> i32 {
    unsafe {
        // copy the contents of memory #`src_addr_mem_idx` cells:
        // `src_addr_mem_ptr, src_addr_mem_ptr + 1, ..., src_addr_mem_ptr + 31` into register `256:0`
        mem_to_reg_copy(src_addr_mem_idx, src_addr_mem_ptr, 32, 256, 0);

        // read the balance stored under register `256:0`
        let src_balance: i64 = get_balance_from_reg(256, 0);

        if src_balance < amount {
            // source has not enough balance
            return -1;
        }

        // copy the contents of memory #`dst_addr_mem_ptr` cells:
        // `dst_addr_mem_ptr, dst_addr_mem_ptr + 1, ..., dst_addr_mem_ptr + 31`
        // into register `1` (of type `256 bits`)
        mem_to_reg_copy(dst_addr_mem_idx, dst_addr_mem_ptr, 32, 256, 1);

        // read the balance stored under register `256:1`
        let dst_balance: i64 = get_balance_from_reg(256, 1);

        let new_src_balance: i64 = src_balance - amount;
        let new_dst_balance: i64 = dst_balance + amount;

        // set new balance `new_src_balance`, to the address under register `256:0`
        set_balance_from_reg(256, 0, new_src_balance);

        // set new balance `new_dst_balance`, to the address under register `256:1`
        set_balance_from_reg(256, 1, new_dst_balance);

        // Copying page `0`, cells: `0..8` into register `64:0`
        storage_read_to_reg(0, 0, 8, 64, 0);

        // Increment the integer under register `64:0`
        let counter = reg_read_le_i64(64, 0);
        reg_write_le_i64(counter + 1, 64, 0);

        // persisting register `64:0` back to app-storage (page `0`, cells: `0..8`)
        storage_write_from_reg(64, 0, 8, 0, 0);
    }

    // success
    return 0;
}
