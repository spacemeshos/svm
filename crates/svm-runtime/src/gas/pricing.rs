/// Updates current running `App`'s `left gas`.
#[macro_export]
macro_rules! use_gas {
    ($vmcall:expr, $ctx:expr) => {{
        use crate::helpers;

        let gas = 10;
        helpers::use_gas($ctx, gas);
    }};
}
