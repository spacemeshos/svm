#[link(wasm_import_module = "svm")]
extern "C" {
    fn svm_transfer(dst_ptr: u32, amount: u64);

    fn svm_host_get64(field_idx: u32) -> u64;

    fn svm_host_get(field_idx: u32, mem_ptr: u32);

    fn svm_calldata_offset() -> u32;

    fn svm_calldata_len() -> u32;

    fn svm_get32(var_id: u32) -> u32;

    fn svm_get64(var_id: u32) -> u64;

    fn svm_set32(var_id: u32, value: u32);

    fn svm_set64(var_id: u32, value: u64);

    fn svm_store160(mem_ptr: u32, var_id: u32);

    fn svm_load160(var_id: u32, mem_ptr: u32);

    fn svm_log(msg_ptr: u32, mem_len: u32, msg_code: u32);
}
