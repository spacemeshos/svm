use crate::ctx::SvmCtx;
use crate::{helpers, use_gas};

use wasmer_runtime::Ctx as WasmerCtx;

pub fn calldata_ptr(ctx: &mut WasmerCtx) -> i32 {
    use_gas!("calldata_ptr", ctx);

    calldata(ctx).0
}

pub fn calldata_len(ctx: &mut WasmerCtx) -> i32 {
    use_gas!("calldata_len", ctx);

    calldata(ctx).1
}

#[inline]
fn calldata(ctx: &mut WasmerCtx) -> (i32, i32) {
    let svm_ctx = helpers::wasmer_data_svm(ctx.data);

    svm_ctx.get_calldata()
}
