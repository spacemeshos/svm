use std::collections::HashMap;
use std::sync::Mutex;

use svm_types::Type;

use lazy_static::lazy_static;

lazy_static! {
    static ref TYPES: Mutex<HashMap<Type, usize>> = Mutex::new(HashMap::new());
    static ref REV_TYPES: Mutex<HashMap<usize, Type>> = Mutex::new(HashMap::new());
}

#[must_use]
pub fn interned_type(ty: Type) -> usize {
    let mut types = TYPES.lock().unwrap();

    let ty_num = types.get(&ty);

    match ty_num {
        Some(num) => *num,
        None => {
            let ty_num = types.len() + 1;

            types.insert(ty, ty_num);

            let mut rev_types = REV_TYPES.lock().unwrap();
            rev_types.insert(ty_num, ty);

            ty_num
        }
    }
}

#[must_use]
pub fn interned_type_rev(interned: usize) -> Option<Type> {
    let rev_types = REV_TYPES.lock().unwrap();

    rev_types.get(&interned).copied()
}
