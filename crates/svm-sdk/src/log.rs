include!("externs.rs");

/// Log the string `msg` along with code `code` into the running App logs.
pub fn log(msg: &str, code: u8) {
    let ptr = msg.as_ptr() as u32;
    let len = msg.len() as u32;

    unsafe { svm_log(ptr, len, code as u32) }
}
