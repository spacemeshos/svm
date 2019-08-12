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
        // `src_addr_mem_ptr, src_addr_mem_ptr + 1, ..., src_addr_mem_ptr + 31`
        // into register `0`
        mem_to_reg_copy(src_addr_mem_idx, src_addr_mem_ptr, 32, 0);

        // register `1` will hold the balance of `source address` (stored under register `0`)
        get_balance_into_reg(0, 1);

        // assert that `source account` has enough balance (i.e at least `amount`)
        // in other words, assert that register `0` > `amount` (little-endian compare)
        if register_le_ucmp_u64(1, amount) < 0 {
            // failure
            return -1;
        }

        // copy the contents of memory #`dst_addr_mem_ptr` cells:
        // `dst_addr_mem_ptr, dst_addr_mem_ptr + 1, ..., dst_addr_mem_ptr + 31`
        // into register `2`
        mem_to_reg_copy(dst_addr_mem_idx, dst_addr_mem_ptr, 32, 2);

        // register `3` will hold the balance of `destination address` (stored under register `2`)
        get_balance_into_reg(2, 3);

        // current registers snapshot:
        // register `0` contains `source address`
        // register `1` contains `source balance`
        // register `2` contains `destination address`
        // register `3` contains `destination balance`

        // `reg1 = reg1 - amount`
        register_le_usub_u64(1, amount, 1);

        // `reg3 = reg3 + amount`
        register_le_uadd_u64(3, amount, 3);

        // copy to the address under register `0`, the balance stored under register `1`
        set_balance_from_reg(0, 1);

        // copy to the address under register `2`, the balance stored under register `3`
        set_balance_from_reg(2, 3);

        // Copying slice `0` (page `0`, cells: `0..8`) into register `0`
        storage_read_to_reg(0, 0, 0, 8, 0);

        // `reg0 = reg0 + 1`
        register_le_uadd_u64(0, 1, 0);

        // persisting register `0` back to contract storage (slice `0`, page `0`, cells: `0..8`)
        storage_write_from_reg(0, 8, 0, 0, 0);
    }

    // success
    return 0;
}
