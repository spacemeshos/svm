use std::collections::HashMap;
use std::sync::Mutex;

use svm_types::Type;

use lazy_static::lazy_static;

lazy_static! {
    static ref TYPES: Mutex<HashMap<Type, usize>> = Mutex::new(HashMap::new());
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