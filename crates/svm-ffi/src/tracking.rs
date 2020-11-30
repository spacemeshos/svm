use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;

use crate::types::TypeIdOrStr;

lazy_static! {
    static ref STATS: Mutex<HashMap<usize, i32>> = Mutex::new(HashMap::new());
    static ref ENABLED: Mutex<bool> = Mutex::new(false);
    static ref TEST: Mutex<()> = Mutex::new(());

    // `TypeId` interning
    static ref TYPES: Mutex<HashMap<TypeIdOrStr, usize>> = Mutex::new(HashMap::new());
    static ref REV_TYPES: Mutex<HashMap<usize, TypeIdOrStr>> = Mutex::new(HashMap::new());
}

#[must_use]
pub fn interned_type<T: 'static>() -> usize {
    let ty = TypeIdOrStr::of::<T>();

    interned_type_1(ty)
}

#[must_use]
pub fn interned_type_1(ty: TypeIdOrStr) -> usize {
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
pub fn num_type(num: usize) -> Option<TypeIdOrStr> {
    let rev_types = REV_TYPES.lock().unwrap();

    rev_types.get(&num).copied()
}

#[must_use]
pub fn start() -> MutexGuard<'static, ()> {
    let lock = TEST.lock().unwrap();

    enable();
    clear();

    lock
}

pub fn end(guard: MutexGuard<'static, ()>) {
    disable();

    drop(guard);
}

#[must_use]
#[cfg(test)]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<usize, i32>>> {
    if is_enabled() {
        let lock = STATS.lock().unwrap();

        Some(lock)
    } else {
        None
    }
}

#[must_use]
#[cfg(not(test))]
pub fn acquire_stats() -> Option<MutexGuard<'static, HashMap<usize, i32>>> {
    let lock = STATS.lock().unwrap();

    Some(lock)
}

#[must_use]
pub fn acquire_enabled() -> MutexGuard<'static, bool> {
    ENABLED.lock().unwrap()
}

fn enable() {
    enable_disable(true);
}

fn disable() {
    enable_disable(false);
}

fn enable_disable(value: bool) {
    let mut enabled = acquire_enabled();

    *enabled = value;
}

#[cfg(test)]
pub fn is_enabled() -> bool {
    let enabled = acquire_enabled();

    *enabled == true
}

#[cfg(not(test))]
#[inline]
fn is_enabled() -> bool {
    true
}

fn clear() {
    let stats = acquire_stats();

    if let Some(mut stats) = stats {
        stats.clear();
    }
}

pub fn snapshot() -> HashMap<&'static str, i32> {
    let stats = acquire_stats().unwrap();

    let rev_types = REV_TYPES.lock().unwrap();

    let mut snapshot = HashMap::new();

    for (ty, count) in stats.iter() {
        let ty = rev_types.get(&ty);

        match ty {
            None => {
                snapshot.insert("[MISSING]", *count);
            }
            Some(ty) => {
                let ty_name = match *ty {
                    TypeIdOrStr::TypeId(_, name) => name,
                    TypeIdOrStr::Str(ty) => ty,
                };

                snapshot.insert(ty_name, *count);
            }
        };
    }

    snapshot
}

pub fn increment_live<T: 'static>() {
    let ty = TypeIdOrStr::of::<T>();

    increment_live_2(ty)
}

pub fn increment_live_1(ty: TypeId, name: &'static str) {
    increment_live_2(TypeIdOrStr::TypeId(ty, name))
}

pub fn increment_live_2(ty: TypeIdOrStr) {
    if is_enabled() {
        let mut stats = acquire_stats().unwrap();
        let ty = interned_type_1(ty);

        let entry = stats.entry(ty).or_insert(0);
        *entry += 1;
    }
}

pub fn decrement_live<T: 'static>() {
    let ty = TypeIdOrStr::of::<T>();

    decrement_live_1(ty);
}

pub fn decrement_live_1(ty: TypeIdOrStr) {
    let ty = interned_type_1(ty);

    decrement_live_2(ty);
}

pub fn decrement_live_2(ty: usize) {
    if is_enabled() {
        let mut stats = acquire_stats().unwrap();

        let entry = stats.entry(ty).or_insert(0);
        *entry -= 1;
    }
}

pub fn live_count<T: 'static>() -> i32 {
    let ty = TypeIdOrStr::of::<T>();
    let ty = interned_type_1(ty);

    live_count_2(ty)
}

pub fn live_count_1(ty: TypeId, name: &'static str) -> i32 {
    let ty = TypeIdOrStr::TypeId(ty, name);
    let ty = interned_type_1(ty);

    live_count_2(ty)
}

fn live_count_2(ty: usize) -> i32 {
    if is_enabled() {
        let stats = acquire_stats().unwrap();

        match stats.get(&ty) {
            None => 0,
            Some(count) => *count,
        }
    } else {
        0
    }
}

pub fn total_live_count() -> i32 {
    if is_enabled() {
        let stats = acquire_stats().unwrap();

        stats.iter().map(|(_ty, count)| count).sum()
    } else {
        0
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     struct A;

//     struct B;

//     #[test]
//     fn tracks_by_type() {
//         let l = track_start();

//         assert_eq!(total_live_count(), 0);

//         increment_live::<A>();
//         increment_live::<A>();
//         increment_live::<B>();

//         assert_eq!(live_count::<A>(), 2);
//         assert_eq!(live_count::<B>(), 1);
//         assert_eq!(total_live_count(), 3);

//         decrement_live::<A>();
//         decrement_live::<B>();

//         assert_eq!(live_count::<A>(), 1);
//         assert_eq!(live_count::<B>(), 0);
//         assert_eq!(total_live_count(), 1);

//         decrement_live::<A>();

//         assert_eq!(live_count::<A>(), 0);
//         assert_eq!(live_count::<B>(), 0);
//         assert_eq!(total_live_count(), 0);

//         track_end(l);
//     }
// }
