use wasmer_middleware_common::metering::{get_points_used_ctx, set_points_used_ctx};
use wasmer_runtime::Ctx as WasmerCtx;

/// Decreases the left gas with `gas` units.
///
/// # Panics
///
/// Panics `Out of Gas` in case there is no sufficient gas left.
#[inline]
pub fn use_gas(ctx: &mut WasmerCtx, gas: u64, gas_limit: u64) {
    let used_gas = get_points_used_ctx(ctx);
    let new_used_gas = used_gas + gas;

    if new_used_gas <= gas_limit {
        set_points_used_ctx(ctx, new_used_gas);
    } else {
        set_points_used_ctx(ctx, gas_limit);

        panic!("Out of Gas");
    }
}
