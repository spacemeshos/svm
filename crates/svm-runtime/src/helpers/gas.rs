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
    let left_gas = get_points_used_ctx(ctx);

    if left_gas < gas {
        let svm_ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(ctx.data) };

        svm_ctx.reached_oog = true;

        panic!("Out of Gas");
    }

    set_points_used_ctx(ctx, left_gas - gas);
}
