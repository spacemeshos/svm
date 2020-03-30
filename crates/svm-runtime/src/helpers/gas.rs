use crate::ctx::SvmCtx;

use wasmer_middleware_common::metering::{get_points_used_ctx, set_points_used_ctx};
use wasmer_runtime::Ctx as WasmerCtx;

/// Decreases the left gas with `gas` units.
///
/// # Panics
///
/// Panics `Out of Gas` in case there is no sufficient gas left.
#[inline]
pub fn use_gas(ctx: &mut WasmerCtx, gas: u64) {
    let used_gas = get_points_used_ctx(ctx);
    let new_used_gas = used_gas + gas;

    let svm_ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(ctx.data) };
    let gas_limit = svm_ctx.gas_limit;

    if new_used_gas <= gas_limit {
        set_points_used_ctx(ctx, new_used_gas);
    } else {
        set_points_used_ctx(ctx, gas_limit);
        svm_ctx.reached_oog = true;

        panic!("Out of Gas");
    }
}
