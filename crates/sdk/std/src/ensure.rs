/// The `ensure!` macro is intended to be used within written SVM Templates.
/// The macro is very similar to the `assert` macro used for writing tests.
/// Aborting in case the exercised expression isn't satisfied,
/// the passed `msg` will be logged-in first.
///
/// That log entry could be later retrieved and inspected since it will part of the
/// logs associated with the SVM transaction receipt.
///

#[macro_export]
macro_rules! ensure {
    ($expr:expr) => {{
        let satisfied = $expr;

        if !satisfied {
            $crate::panic()
        }
    }};
    ($expr:expr, $data:expr) => {{
        let satisfied = $expr;

        if !satisfied {
            $crate::log($data);

            panic()
        }
    }};
}
