use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use super::interning;

use svm_types::Type;

use lazy_static::lazy_static;

lazy_static! {
    static ref STATS: Mutex<HashMap<usize, i32>> = Mutex::new(HashMap::new());
}

#[must_use]
pub fn acquire() -> MutexGuard<'static, HashMap<usize, i32>> {
    STATS.lock().unwrap()
}

#[allow(dead_code)]
pub fn release(_guard: MutexGuard<'static, HashMap<usize, i32>>) {
    //
}

pub fn snapshot() -> HashMap<&'static str, i32> {
    let stats = acquire();

    let mut snapshot = HashMap::new();

    for (interned_ty, count) in stats.iter() {
        let ty = interning::interned_type_rev(*interned_ty);

        match ty {
            None => {
                snapshot.insert("[**Unknown Type**]", *count);
            }
            Some(ty) => {
                let ty_name = match ty {
                    Type::TypeId(_, name) => name,
                    Type::Str(ty) => ty,
                };

                snapshot.insert(ty_name, *count);
            }
        };
    }

    snapshot
}

pub fn increment_live<T: 'static>() {
    let ty = Type::of::<T>();

    increment_live_2(ty)
}

pub fn increment_live_1(ty: std::any::TypeId, name: &'static str) {
    increment_live_2(Type::TypeId(ty, name))
}

pub fn increment_live_2(ty: Type) {
    let mut stats = acquire();
    let ty = interning::interned_type_1(ty);

    let entry = stats.entry(ty).or_insert(0);
    *entry += 1;
}

pub fn decrement_live<T: 'static>() {
    let ty = Type::of::<T>();

    decrement_live_1(ty);
}

pub fn decrement_live_1(ty: Type) {
    let ty = interning::interned_type_1(ty);

    decrement_live_2(ty);
}

pub fn decrement_live_2(ty: usize) {
    let mut stats = acquire();

    let entry = stats.entry(ty).or_insert(0);
    *entry -= 1;

    if *entry == 0 {
        stats.remove(&ty);
    }
}

pub fn live_count<T: 'static>() -> i32 {
    let ty = Type::of::<T>();
    let ty = interning::interned_type_1(ty);

    live_count_2(ty)
}

pub fn live_count_1(ty: std::any::TypeId, name: &'static str) -> i32 {
    let ty = Type::TypeId(ty, name);
    let ty = interning::interned_type_1(ty);

    live_count_2(ty)
}

fn live_count_2(ty: usize) -> i32 {
    let stats = acquire();

    match stats.get(&ty) {
        None => 0,
        Some(count) => *count,
    }
}

pub fn total_live() -> i32 {
    let stats = acquire();

    stats.iter().map(|(_ty, count)| count).sum()
}
