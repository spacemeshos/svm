/// Since we're not using the standard Result (`core::result::Result`)
/// We can't use the `?` operator for injecting immediate return from a function in case of an `Err(..)`  
///
/// So instead, we're adding a macro named `safe_try` that will function very similarly to the `?` of standard `Result`
#[macro_export]
macro_rules! safe_try {
    ($expr:expr) => {{
        use svm_sdk_std::Result;

        let result = $expr;

        if (result.is_ok()) {
            result.unwrap()
        } else {
            let err = result.unwrap_err();

            return Result::Err(err);
        }
    }};
}
