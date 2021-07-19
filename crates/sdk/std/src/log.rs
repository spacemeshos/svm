extern "C" {
    #[link_section = "svm"]
    fn svm_log(msg_ptr: u32, msg_len: u32, code: u32);
}

/// Log the string `msg` along with code `code`.
pub fn log(msg: &str, code: u8) {
    let ptr = msg.as_ptr() as u32;
    let len = msg.len() as u32;

    unsafe { svm_log(ptr, len, code as u32) }
}
