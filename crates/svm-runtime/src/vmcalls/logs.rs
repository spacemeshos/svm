use crate::ctx::SvmCtx;
use crate::{helpers, use_gas};

use svm_types::receipt::Log;
use wasmer_runtime::Ctx as WasmerCtx;

pub fn log(ctx: &mut WasmerCtx, msg_ptr: u32, msg_len: u32, code: u32) {
    use_gas!("log", ctx);

    let start = msg_ptr as usize;
    let end = start + msg_len as usize;

    let msg: Vec<u8> = ctx.memory(0).view()[start..end]
        .iter()
        .map(|cell| cell.get())
        .collect();

    let log = Log {
        msg,
        code: code as u8,
    };

    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(ctx.data) };

    svm_ctx.logs.push(log);
}
