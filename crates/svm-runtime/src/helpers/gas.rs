use wasmer_middleware_common::metering::{get_points_used_ctx, set_points_used_ctx};
use wasmer_runtime::Ctx as WasmerCtx;
use wasmer_runtime::Instance;

use crate::{
    ctx::SvmCtx,
    gas::{MaybeGas, OOGError},
};

/// Decreases the left gas with `gas` units.
///
/// # Panics
///
/// Panics `Out of Gas` in case there is no sufficient gas left.
#[inline]
pub fn wasmer_use_gas(ctx: &mut WasmerCtx, gas: u64, gas_limit: u64) {
    let used_gas = get_points_used_ctx(ctx);
    let new_used_gas = used_gas + gas;

    if new_used_gas <= gas_limit {
        set_points_used_ctx(ctx, new_used_gas);
    } else {
        set_points_used_ctx(ctx, gas_limit);

        panic!("Out of Gas");
    }
}

/// On success returns the amount of gas used during App's execution.
/// Of failure returs `OOGError` (Out-of-Gas).
pub fn wasmer_gas_used(instance: &Instance) -> Result<MaybeGas, OOGError> {
    let wasmer_ctx = instance.context();

    let svm_ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(wasmer_ctx.data) };

    if svm_ctx.gas_metering {
        let gas_used = get_points_used_ctx(&wasmer_ctx);

        if gas_used <= svm_ctx.gas_limit {
            let gas_used = MaybeGas::with(gas_used);

            Ok(gas_used)
        } else {
            Err(OOGError {})
        }
    } else {
        Ok(MaybeGas::new())
    }
}
