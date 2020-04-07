/// Updates current running `App`'s `left gas`.
#[macro_export]
macro_rules! use_gas {
    // buffer
    ("buffer_create", $wasmer_ctx:expr) => {{
        //
    }};
    ("buffer_kill", $wasmer_ctx:expr) => {{
        //
    }};
    ("buffer_freeze", $wasmer_ctx:expr) => {{
        //
    }};
    ("buffer_copy_to_storage", $wasmer_ctx:expr) => {{
        //
    }};
    ("buffer_copy_to_reg", $wasmer_ctx:expr) => {{
        //
    }};

    // host-ctx
    ("host_ctx_read_into_reg", $wasmer_ctx:expr) => {{
        //
    }};
    ("host_ctx_read_i32_le", $wasmer_ctx:expr) => {{
        //
    }};
    ("host_ctx_read_i32_be", $wasmer_ctx:expr) => {{
        //
    }};
    ("host_ctx_read_i64_le", $wasmer_ctx:expr) => {{
        //
    }};
    ("host_ctx_read_i64_be", $wasmer_ctx:expr) => {{
        //
    }};

    // register
    ("reg_set_i32_be", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_set_i32_le", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_set_i64_be", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_set_i64_le", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_push", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_pop", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_cmp", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_set", $wasmer_ctx:expr) => {{
        //
    }};

    // storage
    ("mem_to_reg_copy", $wasmer_ctx:expr) => {{
        //
    }};
    ("reg_to_mem_copy", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_read_to_reg", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_read_to_mem", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_write_from_mem", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_write_from_reg", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_write_i32_be", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_write_i32_le", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_write_i64_be", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_write_i64_le", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_read_i32_be", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_read_i32_le", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_read_i64_be", $wasmer_ctx:expr) => {{
        //
    }};
    ("storage_read_i64_le", $wasmer_ctx:expr) => {{
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
