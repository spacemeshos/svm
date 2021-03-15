#[macro_export]
macro_rules! safe_try {
    ($expr:expr) => {{
        let result = $expr;

        if (result.is_ok()) {
            result.unwrap()
        } else {
            let err = result.unwrap_err();

            return Result::Err(err);
        }
    }};
}
