#![allow(unused)]

svm_extern::include_extern_storage!();

fn execute(src_addr_mem_ptr: i32, dst_addr_mem_ptr: i32, amount: i64) {
    // Copy the contents of `src_addr_mem_ptr, src_addr_mem_ptr + 1, ..., src_addr_mem_ptr + 31` into register `0`
    // unsafe {
    //     mem_to_reg_copy(src_addr_mem_ptr, 32, 0);
    // }

    // assert that `source account` has enough balance
    // let src_balance: Vec<u8> = read_reg(0);
    // assert_gte(src_balance, amount);

    // Copy the contents of `dst_addr_mem_ptr, dst_addr_mem_ptr + 1, ..., dst_addr_mem_ptr + 31` into register `1`
    // unsafe {
    //     mem_to_reg_copy(dst_addr_mem_ptr, 32, 1);
    // }

    // let dst_balance: Vec<u8> = read_reg(1);
    // / let src_new_balance = svm_sub_balance(src_balance, amount);
    // let dst_new_balance = svm_add_balance(dst_balance, amount);
}
