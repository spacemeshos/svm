use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use svm_types::Type;

// Since `svm_byte_array` is exchanged between `Rust` and external-code (i.e FFI code)
// we can't use `Type` for `type_id`.
//
// That's why we apply the interning technique for `Type` (`svm_types::Type`).
// By doing that we can use its interned representation in `svm_byte_array`.
//
// We also implement reverse-interpretation. Give an interned type, we can relate it to its Rust `Type` value.
lazy_static! {
    /// Maps each `Type` to its interned value
    static ref TYPES: Mutex<HashMap<Type, usize>> = Mutex::new(HashMap::new());

    /// Maps each interned value to its origin `Type`
    static ref REV_TYPES: Mutex<HashMap<usize, Type>> = Mutex::new(HashMap::new());
}

/// Returns the interned value of `Type`.
/// If there is already an interned value it's returned,
/// Otherwise, an interned value is generated for `ty`, stored for future-use and returned.
#[must_use]
pub fn interned_type(ty: Type) -> usize {
    let mut types = TYPES.lock().unwrap();

    let interned = types.get(&ty);

    match interned {
        Some(n) => *n,
        None => {
            let interned = types.len() + 1;

            types.insert(ty, interned);

            let mut rev_types = REV_TYPES.lock().unwrap();
            rev_types.insert(interned, ty);

            interned
        }
    }
}

/// Given an interned `Type` (given as integer) returns its associated `Type`.
#[must_use]
pub fn interned_type_rev(interned: usize) -> Option<Type> {
    let rev_types = REV_TYPES.lock().unwrap();

    rev_types.get(&interned).copied()
}
