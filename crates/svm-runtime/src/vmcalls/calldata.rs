use crate::ctx::SvmCtx;
use crate::{helpers, use_gas};

use wasmer_runtime::Ctx as WasmerCtx;

pub fn calldata_ptr(ctx: &mut WasmerCtx) -> i32 {
    use_gas!("calldata_ptr", ctx);

    calldata(ctx).0 as i32
}

pub fn calldata_len(ctx: &mut WasmerCtx) -> i32 {
    use_gas!("calldata_len", ctx);

    calldata(ctx).1 as i32
}

#[inline]
fn calldata(ctx: &mut WasmerCtx) -> (usize, usize) {
    let svm_ctx = helpers::wasmer_data_svm(ctx.data);

    svm_ctx.get_calldata()
}
