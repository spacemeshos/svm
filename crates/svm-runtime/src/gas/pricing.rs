/// Updates current running `App`'s `left gas`.
#[macro_export]
macro_rules! use_gas {
    ("get64", $wasmer_ctx:expr) => {{
        //
    }};
    ("set64", $wasmer_ctx:expr) => {{
        //
    }};
    ("host_get64", $wasmer_ctx:expr) => {{
        //
    }};

    ($wasmer_ctx:expr) => {{
        use crate::ctx::SvmCtx;

        let ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>($wasmer_ctx.data) };

        if ctx.gas_metering {
            // TODO: hardcode the `gas` pricing for each vmcall.
            let gas = 10;

            helpers::wasmer_use_gas($wasmer_ctx, gas, ctx.gas_limit);
        }
    }};
}
