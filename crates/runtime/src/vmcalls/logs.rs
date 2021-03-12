use crate::{use_gas, Context};

use svm_types::Log;

/// Logs the log entry given.
///
/// It's string message sits in memory starting from offset `msg_ptr` and its length is `msg_len`.
/// The log entry numeric code is given via parameter `code`.
pub fn log(ctx: &Context, msg_ptr: u32, msg_len: u32, code: u32) {
    use_gas!("log", ctx);

    let start = msg_ptr as usize;
    let end = start + msg_len as usize;

    let msg: Vec<u8> = {
        let borrow = ctx.borrow();
        let memory = borrow.get_memory();

        memory.view()[start..end]
            .iter()
            .map(|cell| cell.get())
            .collect()
    };

    let log = Log {
        msg,
        code: code as u8,
    };

    ctx.borrow_mut().logs.push(log);
}
