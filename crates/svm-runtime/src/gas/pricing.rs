/// Updates current running `App`'s `left gas`.
#[macro_export]
macro_rules! use_gas {
    ("get32", $ctx:expr) => {{
        //
    }};
    ("set32", $ctx:expr) => {{
        //
    }};
    ("get64", $ctx:expr) => {{
        //
    }};
    ("set64", $ctx:expr) => {{
        //
    }};
    ("load160", $ctx:expr) => {{
        //
    }};
    ("store160", $ctx:expr) => {{
        //
    }};
    ("host_get64", $ctx:expr) => {{
        //
    }};
    ("calldata_offset", $ctx:expr) => {{
        //
    }};
    ("calldata_len", $ctx:expr) => {{
        //
    }};
    ("set_returndata", $ctx:expr) => {{
        //
    }};
    ("log", $ctx:expr) => {{
        //
    }};

    ($ctx:expr) => {{
        use crate::Context

        if ctx.gas_metering {
            // TODO: hardcode the `gas` pricing for each vmcall.

        }
    }};
}
