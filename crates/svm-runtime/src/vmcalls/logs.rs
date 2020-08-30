use crate::{helpers, use_gas, Context};

use svm_types::receipt::Log;

pub fn log(ctx: &mut Context, msg_ptr: u32, msg_len: u32, code: u32) {
    use_gas!("log", ctx);

    let start = msg_ptr as usize;
    let end = start + msg_len as usize;

    let borrow = ctx.borrow();
    let memory = borrow.get_memory();

    let msg: Vec<u8> = memory.view()[start..end]
        .iter()
        .map(|cell| cell.get())
        .collect();

    let log = Log {
        msg,
        code: code as u8,
    };

    ctx.borrow_mut().logs.push(log);
}
