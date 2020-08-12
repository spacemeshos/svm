/// Creates a new `Nibble`.
#[macro_export]
macro_rules! nib {
    ($val:expr) => {{
        use $crate::Nibble;

        Nibble::new($val)
    }};
}
